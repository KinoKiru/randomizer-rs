#!/bin/sh
sea-orm-cli generate entity --with-serde both -o ./entity/src --lib

echo ./entity/src/lib.rs