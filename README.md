# `cid_oracle`

`cid_oracle` scans the Filecoin blockchain for DealProposal messages, and extracts each `PieceCID` <-> `PayloadCID` mapping.  It saves the tuple `{Message CID, PieceCID, Payload CID}` to a SQL database.

Next step:  add a web front end and make these mappings available over an HTTP API to anyone.

## Compiling and Running

To compile and run `cid_oracle`:

```
cargo run -p cid_oracle -- --endpoint="http://lotus1:1234/rpc/v0"
```

Make sure to change the endpoint to a Lotus where you have made [the necessary `config.toml` changes](https://github.com/mgoelzer/lotus_client_rs/blob/master/README.md#configuring-a-lotus-machine).

## Configuration

You can pass all the command line arguments you need (see `--help` for a run down).  Optionally, you may choose to copy [config-example.toml](/config-example.toml) to `$HOME/cid-oracle/config.toml` and uncomment lines in there to modify the defaults.

`--endpoint` is the only option you **must** configure individually.

## Data

`$HOME/cid-oracle/` is also the location of the temporary sqlite3 database that is written to as content CIDs are discovered.  The production version of this program will use a more scalable database backend, and this paragraph will be removed.

## Contributing

Contributions are welcome.  Check out the [issues](/issues) for a start.

## License

Dual-licensed under [MIT](https://github.com/filecoin-project/lotus/blob/master/LICENSE-MIT) + [Apache 2.0](https://github.com/filecoin-project/lotus/blob/master/LICENSE-APACHE)