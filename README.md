

## api
[api-doc](./api-doc.md)



## todo
ignore streamOptions
```rust
if let Some(ref local_var_str) = stream_options {
  local_var_req_builder =
          local_var_req_builder.query(&[("streamOptions", &local_var_str.to_string())]);
}
```