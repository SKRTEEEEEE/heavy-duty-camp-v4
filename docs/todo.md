# Process

### Use Ubuntu WSL
**_For windows_**
- [Solana install tutorial](https://solana.com/es/docs/intro/installation)
- [Solana testnet faucet](https://faucet.solana.com/)
#### Requisitos
- rust: `rustc --version`
- solana CLI: `solana --version`
- anchor version manager: `avm --version`
- anchor: `anchor --version`
- node: `node --version`
### Config solana CLI
- configuración (**cluster**): `solana config get`
- cambiar configuracion (**cluster**):
    ```
    solana config set --url mainnet-beta
    solana config set --url devnet
    solana config set --url localhost
    solana config set --url testnet
    ```
    ```
    solana config set -um    # For mainnet-beta
    solana config set -ud    # For devnet
    solana config set -ul    # For localhost
    solana config set -ut    # For testnet
    ```
- billetera: `solana address`
    - crear: `solana-keygen new`

#### airdrop devnet/testnet
_debes estar en una de las redes de prueba_
- balance: `solana balance`
- airdrop: `solana airdrop <number_to_airdrop>`

#### validador local
_para ejecutar pruebas en tu maquina local, sin interaccion con las redes desplegadas, utilizar esta(mas rapida, sin limite de SOL)_
**ejecutar en una terminal nueva de Ubuntu**
- levantar: `solana-test-validator`

    _una vez este corriendo el servidor, recordar cambiar el cluster para utilizar-lo:_ `solana config set -ul`

### anchor
#### init
_para iniciar un nuevo proyecto, hemos de entender que este se ha de ejecutar en la WSL (Ubuntu), por lo tanto, o apuntamos a la carpeta del sistema principal (Windows) donde lo vamos a crear, o lo creamos en la WSL._
- base template: `anchor init <nombre_proyecto>`
#### build
_para construir nuestro contrato, debemos: Utilizar una clave pública de ejemplo temporal._
- obtener clave pública de ejemplo: `solana-keygen pubkey ~/my_keypair.json`
    - generar clave pública de ejemplo: `solana-keygen new --outfile ~/my_keypair.json`

_debemos utilizar esta clave a la hora de declarar nuestro id del programa_
```rust
use anchor_lang::prelude::*;

// declaramos el id
declare_id!("<AQUI_IRA_LA_CLAVE_PUBLICA_DE_EJEMPLO>");

// definimos el programa
#[program]
// ...continue
```

- build: `anchor build`

    _si al hacer build, nos aparece este error:_

        error: failed to parse lock file at: /Users/{user}/{project_dir}/anchor/Cargo.lock
        Caused by:
        lock file version 4 requires `-Znext-lockfile-bump
    
    _podemos cambiar la version de nuestro archivo `Cargo.toml` [(toda la info, aquí)](https://github.com/coral-xyz/anchor/issues/3392):_

        version = 3

- deploy: `anchor deploy`