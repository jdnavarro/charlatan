## Charlatan
![CI](https://github.com/jdnavarro/charlatan-server/workflows/CI/badge.svg)

Self hosted client-server podcatcher.

For now this is an experimental project to learn about Rust, TypeScript and
modern React techniques.

### Checking it out

1. Create a dummy `sqlite db` such as:

    ```bash
    $ sqlite3 /tmp/test.sqlite -line ".read schema.sql"
    ```

1. Set up the necessary environment variables. I'm using
   [`direnv`](https://direnv.net/) and this is how my `.envrc` looks:

    ```
    export DATABASE_URL="sqlite:/tmp/test.sqlite"
    export BIND_ADDRESS="0.0.0.0:3030"
    export WEB_DIR="/home/danny/Devel/charlatan/server/web/build/"
    export REACT_APP_API_URL="http://localhost:3030/api"
    export JWT_SECRET="secret"
    ```
    
1. Run the server:

    ``` sh
    $ cargo run --features web
    ```

    This already serves the static assets for the UI, but you can run it
    separately:

    ```sh
    $ cargo run
    $ cd web
    $ yarn start
    ```

1. Register yourself

   The first time you go the web application it will prompt you to register. For
   now you can only register once.
   
   If you need to reset the password you will need to remove the user row in the db:
   ```sh
   $ sqlite3 <sqlite-file> -line "delete from user"
   ```
   
   Only one user is supported for now.

1. Crawl all podcasts in order to obtain the episodes:

   You can obtain the token by going to the web console and copying the `token`
   in `localStorage`.
      
   ```sh
   $ curl -X POST -H "Authorization: Bearer <token>" http://127.0.0.1:3030/crawl
   ```
