# `cid_oracle`

`cid_oracle` scans the Filecoin blockchain for DealProposal messages, and extracts each `PieceCID <=> PayloadCID` mapping.  It saves the tuple `{Message CID, PieceCID, Payload CID}` to a SQL database.  Next step:  add a web front end and make these mappings available over an HTTP API to anyone.

## Why?

Historically, tools that stored and retrieved user data to the Filecoin network exposed primarily or only the hash of the raw data itself, called "Payload CID," "File CID," "Data CID," and other names.  This is roughly equivalent to the sha-256 of your data, but formatted according to [IPFS CID specification](https://docs.ipfs.io/concepts/content-addressing/).  However, Filecoin stores all data as [IPLD DAGs](https://github.com/ipld/specs), and the root hash of this structure, called `Piece CID` or `commP`, was the only on-chain record of what miner is storing that data.

To enable retrieval miners to field requests for a particular Payload CID and be able to discover what storage miner has that piece of data, Filecoin storage deals typically include the Piece CID, Payload CID and miner BLS wallet address in a proposal message signed by the original buyer of the storage.  `cid_oracle` provides random access to these `{Piece CID, Payload CID, MsgCid}` tuples. 

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