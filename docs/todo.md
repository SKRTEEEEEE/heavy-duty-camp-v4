# Data
## config
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
#### Config solana CLI
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

##### airdrop devnet/testnet
_debes estar en una de las redes de prueba_
- balance: `solana balance`
- airdrop: `solana airdrop <number_to_airdrop>`

##### validador local
_para ejecutar pruebas en tu maquina local, sin interaccion con las redes desplegadas, utilizar esta(mas rapida, sin limite de SOL)_
**ejecutar en una terminal nueva de Ubuntu**
- levantar: `solana-test-validator`

    _una vez este corriendo el servidor, recordar cambiar el cluster para utilizar-lo:_ `solana config set -ul`

#### anchor
##### init
_para iniciar un nuevo proyecto, hemos de entender que este se ha de ejecutar en la WSL (Ubuntu), por lo tanto, o apuntamos a la carpeta del sistema principal (Windows) donde lo vamos a crear, o lo creamos en la WSL._
- base template: `anchor init <nombre_proyecto>`
##### build
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
##### deploy
- deploy: `anchor deploy`

## teoria
### [1. Cuentas y programas](./HDC%20v4%20-%20Clase%20#1.pdf)
#### Cuentas
Cada cuenta tiene un tamaño y public key. 

El tamaño viene dado por la cantidad de datos que puede contener. 128 bytes por defecto. 

    👁️ No confundir cuenta con dirección

Para mantener una cuenta hay que pagar la renta proporcional a las fees.

**0.00000348 $SOL por byte por año**

Si depositamos en la cuenta la cantidad de renta para dos años tendremos **rent-exemption**. Evitando asi los 'gastos' de renta.

Hay cuentas ejecutable y no ejecutables.

#### Programas
 En el ecosistema de Solana, los "contratos inteligentes" se denominan programas.

 Un programa es una cuenta que contiene código ejecutable.

 No tienen estado; todos los datos que necesitan para operar se almacenan en cuentas separadas.

- cuenta de un programa: almacena dirección y autoridad
- cuenta de datos ejecutables: código ejecutable
- cuenta buffer: temporal

Hay programas On-Chain (comunidad) y nativos (solana).
#### Links compartidos
- [Modelo de cuentas](https://solana.com/es/docs/core/accounts)
- [Renta/comisiones](https://solana.com/es/docs/core/fees)
- [Solana CLI](https://docs.anza.xyz/es/cli/usage)
### [2. Transacciones y intro a anchor](./HDC%20v4%20-%20Clase%20#2.pdf)
#### Transacciones
Conjunto de instrucciones que interactúan con varios programas (max 1232 bytes)

- [estructura general](./img/transaccion-estructura.png)

- [estructura mensaje](./img/mensaje-estructura.png)

    👁️ _Cada parte del mensaje tiene su propia estructura. Para verlo a fondo diriguete al PDF._

##### Instrucciones
Activan programas. Estructura: program_id, accounts (arreglo de AccountMeta) y data. 

##### Comisiones (fee) de transacción
Cada transacción tiene una tarifa base mínima (0.000005 $SOL) para cubrir costos computacionales.
Requiere al menos una cuenta que firme la transacción y sea mutable. Si hay varias cuentas firmantes mutables, la primera de ellas será la que pagara (fee payer), antes de procesas cualquier instrucción.

- transfer: `solana transfer <address_to_send> <quantity_in_$sol>`

    ⚠️ _si la cartera a la que enviamos nunca ha recibido fondos, nos aparecera un error, debemos incluir la bandera `--allow-unfunded-recipient`_
#### ➕ Extra
##### Configurar devnet en Phantom
- [Para configurar la redes de desarrollo en Phantom, debemos hacer click en el avatar de nuestra cuenta en el header de Phantom:](./img/header-phantom.png)

- [Una vez abierto, hacer click en ajustes, y ahi buscar la seccion `Ajustes para desarrolladores` -> seleccionar `Modo Testnet`](./img/options-phantom.png)

    
#### anchor

Framework de rust, para programas de Solana.
##### macro #[account]
- Definir contexto de una instrucción

    - #[account]: definir cuenta y validar o acceder cuentas dentro del contexto de la instrucción 

        #[account(mut)] -> indica que puede ser manipulada

        #[account(init)] -> inicializar nueva cuenta

    - datos: permitir datos personalizados que serán enviados o recibidos por instrucciones

        #[derive(Account)] -> informa, estructura contiene cuentas necesarias para instrucción

    - funciones:

        #[program]: define el módulo princiapl de un programa, convierte las funciones definidas en puntos de entrada de las instrucciones del programa (función que se invoca cuando una transacción llama a dicha instrucción).

        Punto de entrada: donde se ejecuta la lógica de la instrucción. Utiliza el contexto (cuentas necesarios y datos de entrada), proporcionados por la transacción que invoca la instrucción.
        



