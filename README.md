# Japanese study tracker

Hostname: https://divine-thunder-7423.fly.dev

## Development

Run using docker compose. Before running docker-compose you must set the following values, or copy the env.development.sample file and update it with the right value:
```bash
export WANIKANI_API_TOKEN="<my-api-token>"
export BUNPRO_API_TOKEN="<my-api-token>"
export SATORI_COOKIE="<session-cookie>"
export ANKIWEB_COOKIE="<session-cookie>"
# OR:
cp .env.development.sample .env.development # add the right tokens in
```

```bash
docker compose up --build
```

## Testing
### Backend

All commands are run from the backend directory.

```bash
cargo test
```

### Frontend
All commands are run from the frontend directory.

```bash
npm run test:unit
```

## Deploying

Deploying to fly.io uses the top level Dockerfile. Deploying is done with:
```bash
fly deploy # Note: you need to have access to the project in fly.io to deploy
```

### Testing the deploy image
The deploy image can be build and run with:
```bash
docker build -t deploy-test .
docker run --env-file .env.development -p 3000:3000 -it deploy-test
```