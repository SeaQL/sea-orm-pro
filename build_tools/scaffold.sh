#!/bin/bash
set -e

rm -rf migration/src
mv migration/bak migration/src
find migration/src -type f -name '*.rs' -exec sed -i '' '/\/\/ Start: SeaORM Pro Plus \/\//d' {} +
find migration/src -type f -name '*.rs' -exec sed -i '' '/\/\/ End: SeaORM Pro Plus \/\//d' {} +

mv src/models/user.rs src/models/user.bak
rm src/models/*.rs || true
mv src/models/user.bak src/models/user.rs
echo "pub mod user;" > src/models/mod.rs

sed -i '' '/# Start: Scaffold/,/# End: Scaffold/d' ../sea-orm-pro/pro_admin/dashboard.toml
rm pro_admin/raw_tables/*.toml
rm pro_admin/composite_tables/*.toml

find src -type f -name '*.rs' -exec sed -i '' '/\/\/ Start: SeaORM Pro Plus \/\//d' {} +
find src -type f -name '*.rs' -exec sed -i '' '/\/\/ End: SeaORM Pro Plus \/\//d' {} +
find src -type f -name '*.rs' -exec sed -i '' '/\/\/ Start: Scaffold \/\//,/\/\/ End: Scaffold \/\//d' {} +

rm tests/*.rs