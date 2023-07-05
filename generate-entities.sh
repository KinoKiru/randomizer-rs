#!/bin/sh
sea-orm-cli generate entity --with-serde both -o ./entity/src --lib

echo "pub mod extension;" >> ./entity/src/lib.rs