# FAUNA proof-depth CLI proof

```powershell
cargo run -p fauna-cli -- validate fixtures\proof\proof-depth-fauna.json
cargo run -p fauna-cli -- validate fixtures\proof\proof-depth-fauna-invalid.json
```

The accepted fixture exits 0 with no error findings. The invalid fixture exits
1 with a `duplicate_species_id` finding.