## Charlatan

Self hosted client-server podcatcher.

This is a toy project in order to learn about Rust, TypeScript and modern React.

### Checking it out

1. Create a dummy `sqlite db` such as:

    ```bash
    $ sqlite3 /tmp/test.sqlite -line ".read schema.sql"
    ```

1. Set up the necessary environment variables. I'm using [`direnv`](https://direnv.net/) and this is how my `.envrc` looks:

    ```
    export DATABASE_URL="sqlite:/tmp/test.sqlite"
    export BIND_ADDRESS="0.0.0.0:3030"
    export WEB_DIR="/home/danny/Devel/charlatan/server/web/build/"
    export REACT_APP_API_URL="http://localhost:3030/api"
    ```

1. Run ther server:

    ``` sh
    $ cargo run --features web
    ```
This already serves the static assets for the UI, but you can run the UI separately:
    ```sh
    $ cargo run
    $ cd web
    $ yarn start
    ```

1. You can add a podcast through the UI or by hitting the corresping endpoint

    ``` sh
    $ curl http://127.0.0.1:3030/podcasts -H "Content-Type: application/json" -d '{"url": "http://feeds.feedburner.com/dancarlin/history"}'
    ```

1. Crawl all podcasts in order to obtain the episodes:

    ``` sh
    $ curl -X POST http://127.0.0.1:3030/crawl
    ```
