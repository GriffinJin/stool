use std::fs;
use std::io::Write;
use std::path::Path;

/// 处理 SQL 文件：删除 INSERT 字段列表中的 id（不区分大小写），并删除 VALUES 的首个值。
/// 原地修改，同时生成 `.bak` 备份文件。
pub fn rmid_file(sql_path: &Path) -> Result<usize, String> {
    let content = fs::read_to_string(sql_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    // 备份原文件
    let bak_path = sql_path.with_file_name(format!("{}.bak", sql_path.file_name().unwrap().to_string_lossy()));
    fs::write(&bak_path, &content).map_err(|e| format!("写入备份失败: {}", e))?;

    // 处理内容
    let mut changed_count = 0usize;
    let mut processed = content.clone();

    // 移除字段列表中的 id（处理 INSERT INTO ... ( ... )）
    processed = remove_id_from_insert_columns(&processed, &mut changed_count);

    // 移除 VALUES 每个元组的首个值（处理 VALUES (...) [, (...)]*）
    processed = remove_first_value_from_values_tuples(&processed, &mut changed_count);

    // 写回文件
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(sql_path)
        .map_err(|e| format!("写入文件失败: {}", e))?;
    file.write_all(processed.as_bytes())
        .map_err(|e| format!("写入内容失败: {}", e))?;

    Ok(changed_count)
}

fn remove_id_from_insert_columns(input: &str, changed_count: &mut usize) -> String {
    // 简单解析 INSERT 的列列表：INSERT ... (cols)
    // 逐段扫描，定位 '(' 与 ')'，提取列列表，按逗号分割，移除等于 id 的项。
    let mut out = String::with_capacity(input.len());
    let mut i = 0usize;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        // 查找 "INSERT" 位置（不区分大小写）
        if match_kw_ci(bytes, i, b"insert") {
            // 把 "insert" 之前的内容写入
            out.push_str(&input[i..i + 6]);
            i += 6;
            // 后续直接复制直到遇到第一个 '('，然后尝试解析列列表
            let start = i;
            while i < bytes.len() && bytes[i] != b'(' { i += 1; }
            out.push_str(&input[start..i]);
            if i >= bytes.len() { break; }
            // 现在位于 '('
            out.push('(');
            i += 1;
            let cols_start = i;
            let mut depth = 1i32;
            while i < bytes.len() && depth > 0 {
                if bytes[i] == b'(' { depth += 1; }
                else if bytes[i] == b')' { depth -= 1; }
                i += 1;
            }
            let cols_end = i - 1; // 指向 ')'
            let cols_raw = &input[cols_start..cols_end];
            let cleaned = strip_id_tokens(cols_raw);
            if cleaned != cols_raw { *changed_count += 1; }
            out.push_str(&cleaned);
            out.push(')');
            // 继续
            continue;
        } else {
            out.push(input.as_bytes()[i] as char);
            i += 1;
        }
    }
    // 追加剩余内容
    if i < bytes.len() { out.push_str(&input[i..]); }
    out
}

fn remove_first_value_from_values_tuples(input: &str, changed_count: &mut usize) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0usize;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        if match_kw_ci(bytes, i, b"values") {
            out.push_str(&input[i..i + 6]);
            i += 6;
            // 跳过空白
            while i < bytes.len() && is_space(bytes[i]) { out.push(bytes[i] as char); i += 1; }
            if i < bytes.len() && bytes[i] == b'(' {
                out.push('(');
                i += 1;
                let tuple_start = i;
                // 找到第一个不在字符串中的逗号位置
                let cut = find_first_comma_outside_string(&input[tuple_start..]);
                if let Some(cpos) = cut {
                    let rest = &input[tuple_start + cpos + 1..];
                    // 将去掉首值后的内容直到对应的 ')'
                    let (inside, consumed) = take_until_matching_paren(rest);
                    out.push_str(inside.trim_start());
                    out.push(')');
                    i = tuple_start + cpos + 1 + consumed + 1; // 跳过 ')' 本身
                    *changed_count += 1;
                    // 处理后续的多个元组： , (...)
                    while i < bytes.len() {
                        // 跳过空白和逗号
                        let mut j = i;
                        while j < bytes.len() && is_space(bytes[j]) { j += 1; }
                        if j < bytes.len() && bytes[j] == b',' {
                            out.push(',');
                            j += 1;
                            while j < bytes.len() && is_space(bytes[j]) { out.push(bytes[j] as char); j += 1; }
                            if j < bytes.len() && bytes[j] == b'(' {
                                out.push('(');
                                j += 1;
                                let start2 = j;
                                let cut2 = find_first_comma_outside_string(&input[start2..]);
                                if let Some(cpos2) = cut2 {
                                    let rest2 = &input[start2 + cpos2 + 1..];
                                    let (inside2, consumed2) = take_until_matching_paren(rest2);
                                    out.push_str(inside2.trim_start());
                                    out.push(')');
                                    j = start2 + cpos2 + 1 + consumed2 + 1;
                                    *changed_count += 1;
                                    i = j;
                                    continue;
                                }
                            }
                        }
                        // 非元组或无法继续，停止进一步处理
                        break;
                    }
                    continue;
                }
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    if i < bytes.len() { out.push_str(&input[i..]); }
    out
}

fn strip_id_tokens(cols: &str) -> String {
    let mut out_tokens: Vec<String> = Vec::new();
    for tok in cols.split(',') {
        let t = tok.trim();
        let t_clean = t.trim_matches('`').trim_matches('"').trim_matches('\'');
        if !t_clean.eq_ignore_ascii_case("id") && !t_clean.eq_ignore_ascii_case("`id`") {
            out_tokens.push(t.to_string());
        }
    }
    if out_tokens.is_empty() { String::new() } else { out_tokens.join(", ") }
}

fn match_kw_ci(bytes: &[u8], i: usize, kw: &[u8]) -> bool {
    if i + kw.len() > bytes.len() { return false; }
    for k in 0..kw.len() {
        let b = bytes[i + k];
        let c = kw[k];
        if b.to_ascii_lowercase() != c { return false; }
    }
    true
}

fn is_space(b: u8) -> bool { b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' }

fn find_first_comma_outside_string(s: &str) -> Option<usize> {
    let mut in_squote = false;
    let mut in_dquote = false;
    let mut prev_slash = false;
    for (idx, ch) in s.chars().enumerate() {
        match ch {
            '\'' => { if !in_dquote && !prev_slash { in_squote = !in_squote; } prev_slash = false; },
            '"' => { if !in_squote && !prev_slash { in_dquote = !in_dquote; } prev_slash = false; },
            ',' => { if !in_squote && !in_dquote { return Some(idx); } prev_slash = false; },
            '\\' => { prev_slash = !prev_slash; },
            _ => { prev_slash = false; }
        }
    }
    None
}

fn take_until_matching_paren(s: &str) -> (String, usize) {
    // 返回括号内的内容以及消耗的字符数，直到遇到第一个未被引号包裹的 ')'
    let mut in_squote = false;
    let mut in_dquote = false;
    let mut prev_slash = false;
    let mut buf = String::new();
    for (idx, ch) in s.chars().enumerate() {
        match ch {
            '\'' => { if !in_dquote && !prev_slash { in_squote = !in_squote; } prev_slash = false; buf.push(ch); },
            '"' => { if !in_squote && !prev_slash { in_dquote = !in_dquote; } prev_slash = false; buf.push(ch); },
            ')' => { if !in_squote && !in_dquote { return (buf, idx); } else { buf.push(ch); prev_slash = false; } },
            '\\' => { prev_slash = !prev_slash; buf.push(ch); },
            _ => { buf.push(ch); prev_slash = false; }
        }
    }
    (buf, s.len())
}