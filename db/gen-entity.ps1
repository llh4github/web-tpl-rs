sea-orm-cli generate entity `
  --with-serde both --verbose `
  --model-extra-derives  utoipa::ToSchema `
  -o src/entities
