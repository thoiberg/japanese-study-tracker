# Japanese study tracker

## Development

Run using docker compose. Before running docker-compose you must set the following values, or copy the env.development.sample file and update it with the right value:
```bash
export WANIKANI_API_TOKEN="<my-api-token>"
# OR:
cp .env.development.sample .env.development # app the right tokens in
```

```bash
docker compose up --build
```

## Testing
### Backend
```bash
cargo test
```

### Frontend
```bash
npm run test:unit
```