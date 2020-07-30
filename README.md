# `cid_oracle`

`cid_oracle` scans the Filecoin blockchain for DealProposal messages, and extracts the PieceCID <-> PayloadCID mapping.  It saves the tuple {Message CID, PieceCID, Payload CID} to a SQL database.

Next step:  add a web front end and make these mappings available over an HTTP API to anyone.

## Compiling and Running

To compile and run `cid_oracle`:

```
cargo run -p cid_oracle -- --endpoint="http://lotus1:1234/rpc/v0"
```

Make sure to change the endpoint to a Lotus where you have made [the necessary `config.toml` changes](https://github.com/mgoelzer/lotus_client_rs/blob/master/README.md#configuring-a-lotus-machine).
