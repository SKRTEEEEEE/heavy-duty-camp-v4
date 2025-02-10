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
**Obtener llave publica programa**

_para construir nuestro contrato, debemos: Utilizar una clave pública de ejemplo temporal._
- obtener clave pública de ejemplo: `solana-keygen pubkey ~/my_keypair.json`
    - generar clave pública de ejemplo: `solana-keygen new --outfile ~/my_keypair.json`

**Utilizar llave publica programa**
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
    
    _podemos cambiar la version de nuestro archivo `Cargo.lock` [(toda la info, aquí)](https://github.com/coral-xyz/anchor/issues/3392):_

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
#### ➕ Extra
##### Calcular rent excempt con solana
- rent-excempt: `solana rent <tamaño_cuenta_bytes>`
```
⚠️ Tamaño de la cuenta -> sin contar el overhead (128bytes)
```
```
👁️ Mejor ejecutar el codigo en solana playground, ya que en WSL nos dara menos información
```
    
#### 📄 Links
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
#### ➕ Extra
##### Configurar devnet en Phantom
- [Para configurar la redes de desarrollo en Phantom, debemos hacer click en el avatar de nuestra cuenta en el header de Phantom](./img/header-phantom.png)

- [Una vez abierto, hacer click en ajustes, y ahi buscar la seccion `Ajustes para desarrolladores` -> seleccionar `Modo Testnet`](./img/options-phantom.png)

##### Interactuar con programas en [solpg.io](https://beta.solpg.io/)
- Una vez compilado y desplegado nuestro programa. Podremos interactuar con el, para eso, debemos ir a la [sección de test](./img/test-solpg.png). 
- En la sección `Instructions`, buscar nuestra función de instrucción. Al hacer click, aparecen tres cuentas (contexto de la instruccion).
    - El system_program podemos dejar el que nos pone por defecto: `11111111111111111111111111111111`
    - El 'user' _basado en los nombres de mi ejemplo en: [solpg.io](https://beta.solpg.io/679f6de0cffcf4b13384d60d) o [ejercicios.md](./ejercicios.md#2-definiendo-cuentas-e-intrucciones)_, sera el payer -> una cuenta/`publicKey` con fondos para pagar las fee
    - El 'mensaje_account' _basado en los nombres de mi ejemplo en: [solpg.io](https://beta.solpg.io/679f6de0cffcf4b13384d60d) o [ejercicios.md](./ejercicios.md#2-definiendo-cuentas-e-intrucciones)_, sera la cuenta 'recipiente' -> una cuenta/`publicKey` donde deseamos alojar la estructura definida (en el ejemplo, `valor:String`)

- Para interactuar con distintas wallets, debemos [hacer click en la pestaña de `Wallets`](./img/e.png)


#### 📄 Links
- [macros - rust](https://book.rustlang-es.org/ch19-06-macros)
- [lifetime - rust](https://book.rustlang-es.org/ch10-03-lifetime-syntax)
- [Result - rust](https://book.rustlang-es.org/ch09-02-recoverable-errors-with-result)
- [fn 'functions' - rust](https://book.rustlang-es.org/ch03-03-how-functions-work)
- [Transacciones y instrucciones](https://solana.com/es/docs/core/transactions)

### [3. Introducción tokens](./HDC%20v4%20-%20Clase%20#3.pdf)
En Solana, son conocidos como SPL (Solana Program Library), existen fungibles y no fungibles (principalmente).

[Tiene la misma estructura que cualquier programa, la cual viene dada por el BPF Loader (Programas nativos de Solana)](./img/estructura-tokens.png)
#### `mint account`
Cuenta que almazena la información (suministro total, creador y configuraciones especiales), utilizada para gestionar tokens(almacena la información principal del token). La dirección de esta cuenta es la que identifica al token en la red.

    👁️ No tiene nada que ver con la accion de 'mint' tokens

_La principal funcion es **indicar** el suministro total (impresiones y quemas)_

Siempre será no ejecutable, ya que su función es almacenar información.

_La información extra, que no viene indicada por la estructura de mint account (como imagen, web, etc..) se conoce como metadatos_
##### [Estructura datos `mint account`](./img/datos-mint-account.png)
##### CLI `mint account`
Para crear tokens, utilizamos la libreria de anchor `spl-token`
- crear token (`mint account`): `spl-token create-token`

#### `token account`
Para rastrear la propiedad de cada unidad, relaciona una cantidad de un token especifico con un usuario o propietario.

    🪄 Cada usuario tendra su cuenta para almacenar las cantidades de un token en concreto

El propietario de la token account, será siempre el programa de tokens.

Tienen la misma estructura que cualquier cuenta, pero con una [estructura de datos `token account`](./img/estructura-token-account.png).
##### Tokens en una billetera (wallet)
Crear `token account` para token especifico(`mint account`), que designa a la billetera como el propietario (del `token account`).

[Representación gráfica: recibiendo tokens en una billetera](./img/tokens-in-wallet.png)
##### `ATA`: Cuenta de token asociada
_Vincula wallet con `mint account`_

Se deriva determinísticamente usando la dirección del propietario y la cuenta mint. _PDA: Se crea apartir de dos parametros, siguiendo una estructura(es posible conocer el ATA, si conocemos los parametros)_

[Estructura vinculación datos `ATA` (Cuenta de token asociada)](./img/datos-ata.png)
##### Aumentar el suministro de tokens
Se conoce como 'mint', crea nuevas unidades y las asocia a un `token account`.
##### CLI `token account`
- crear `token account`: `spl-token create-account <direccion_mint_account>`
- mint tokens: `spl-token mint <direccion_mint_account> <cantidad> -- <direccion_token_account>`
```
👁️ La dirección del `token account` debe estar vinculada a la direccion `mint account`
```

#### anchor
Para poder utilizar la lógica del programa de tokens en anchor, debemos **importar la librería `anchor_spl`**
#### 📄 Links
- [Gestion de espacio - anchor](https://www.anchor-lang.com/docs/references/space)
- [Constraints (init_if_needed) - anchor](https://www.anchor-lang.com/docs/references/account-constraints)
- [Tokens](https://solana.com/es/docs/core/tokens)
- [Tipos de las cuentas - anchor](https://www.anchor-lang.com/docs/account-types)

### [4. PDA (Dirección derivada de un programa)](./HDC%20v4%20-%20Clase%20#4.pdf)
#### Cuenta token asociada 
Cuenta token con una dirección específica que es generada utilizando algunas entradas, y que siempre genera la misma dirección para las mismas entradas. Esta nos devuelve una dirección derivada de programas (PDA).
#### PDA (Dirección derivada de programa)
Derivadas de:
- combinación de semillas definidas por usuario -> opcionales
- bump o nonce -> obligatorio
- identificador de un programa -> obligatorio
No poseen llaven privada (ninguna entidad puede generar firma válida para PDA)

Generado con hashing, SHA256, pero con un algoritmo distinto al Ed25519 (algoritmo para generar par de llaves, publicKey - privateKey 'válidos'). Por lo tanto, no pueden participar en operaciones criptograficas, ya que no tienen privateKey.

Sirven para almacenar estados, datos, etc... pero con la particularidad que solo pueden ser gestionadas por un programa, sin que ninguna entidad externa pueda intervenir.

Las PDAs permiten programas gestionen recursos sin necesidad humana, crucial para automatizar.

##### Clave Pública del Programa (Program ID)
Dirección del programa de Solana que desea derivar una PDA.
##### Semillas (seeds)
Una o más cadenas de bytes que sirven como identificadores únicos. (texto, numeros, etc..)
##### Bump Seed
Un número (0-255) que se ajusta para encontrar unadirección válida que no tenga una clave privada asociada.

- Iterar el bump: Se ajusta desde el mas alta(255) hasta el mas bajo, hasta encontrar una dirección válida

- Bump canónico: Primer bump que deriva una PDA válida. Esto genera seguridad. Debemos asegurar-nos que siempre se usa el bump cánonico.

#### anchor
Utilizamos la macro `#[account]`, con los mismos atributos que las otras cuentas(init, etc..) y se definen las seeds.
#### ➕ Extra
##### Derivando una PDA, en WSL
Para poder ejecutar el codigo ts en el proyecto iniciado con `anchor init`, hemos de instalar las siguientes dependencias:
```
npm install --save @solana/web3.js@1
npm i --save-dev @types/node
```
Luego, añadir en el compilerOptions, del archivo tsconfig.json. Types: "node", Lib: "exnext" y "dom".
##### Interactuando con PDAs en [solpg.io](https://beta.solpg.io/)
Para poder interactuar con nuestro programa y llamar a la instrucción que modifica la PDA debemos tener en cuenta lo siguiente:
- La cuenta PDA debe ser calculada por nosotros. Para ello podemos ejecutar el siguiente código en el cliente

    ```ts
    // todas las semillas que se requieran
    const programId = new PublicKey("DZ3rUvPXHzD7TwVwECwC9186997jNFnVEPhb5jL8E9Zg"); //programId, este es el que aparece en el contrato de Rust dentro de declareId!("")
    const auth = new PublicKey("GQAJyEhWNdyN3hLDN659aU5y5jiBU27BS8Acb6iYkrRV"); //authority, en este caso es la cuenta pagadora
    const [PDA, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("contador"),auth.toBuffer()], //El ORDEN DE LAS SEMILLAS IMPORTA!DEBE SER EL MISMO
    programId //el programId no es el systemProgram
    );
    console.log("PDA: ", PDA.toBase58());
    ```

- También podemos crear la semilla dentro de la sección de Test de [solpg.io](https://beta.solpg.io/), para ello debemos clickar en el input de la PDA y seleccionar la opción `from seed`. Una vez clickemos, debemos indicar las semillas si las hay.

- ALGO INTERESANTE: Una vez que se ha creado una cuenta PDA (Program Derived Address) con la instrucción crear_contador, no puedes volver a llamar a esa misma instrucción para crearla otra vez. Esto se debe a que la PDA está determinada por las semillas y el bump generado, y su dirección es única.

    Razones:

    - La PDA es derivada de las semillas: Las semillas y el bump hacen que la PDA sea determinística. Una vez que se genera la PDA con ciertas semillas, esta dirección no cambia. Si intentas crearla de nuevo, las semillas (y probablemente el bump) serán iguales, por lo que el sistema reconocerá que ya existe.

    - La cuenta PDA es única y no puede ser creada más de una vez: Estas semillas siempre derivarán la misma dirección, y como la cuenta ya existe, Anchor y el sistema de Solana no permitirán que se cree una segunda vez.

    - Error en el intento de re-creación: Si intentas volver a ejecutar crear_contador para una PDA que ya existe, obtendrás un error indicando que la cuenta ya está inicializada o que la PDA ya existe.

### [5. CPI (Cross-Program Invocations)](./HDC%20v4%20-%20Clase%20#5.pdf)
Nos permite que nuestro programa llame a instrucciones de otro programa, siempre y cuando viva en la misma red y sepa como se llaman y como invocar dicha instrucción (parecido a las API)

Varia según el tipo de cuentas involucradas: con o sin PDA.

Se pueden invocar instrucciónes continuas, se conoce como **pila de invocación**, y tiene un máximo de 4, haciendo un total de 5 invocaciones.

*Cuando creamos SPL Tokens, estamos invocando una instrucción al system_program (CPI) ❓*

Es crucial verificar los datos a la hora de permitir recibir datos, ya que la SVM no es capaz de validarlos 'previamente'.

*Cuando necesitamos una firma para ejecutar la instrucción, se debe pasar la firma al resto de instrucciones que se llamaran para verificar la autoridad, etc.., pero a la hora de utilizar PDA deberemos enviar las semillas ❓*

#### invoke()
Para CPI basicas. Cuando no necesitamos utilizar una PDA, transacción tradicional con cuentas que poseen llave privada y pueden firmar la instrucción. Recibe los datos de la instrucción que queremos invocar y las cuentas relacionadas (o firmantes en segun que casos).
#### invoke_signed()
Para CPI con PDA. La misma estructura, pero debemos enviar tambien las semillas.
#### anchor
Para utilizar las CPI, debemos utilizar el CpiContext de anchor si es posible para nuestro proyecto. Sino deberemos utilizar las funciones 'nativas'.

Recibe dos parametros, (para las CPI básicas): el id del programa que vamos a invocar y la instrucción del programa que vamos a invocar.

Para CPI con PDA, necesitamos utilizar el metodo .with_signer(), pasandole las semillas.

### [6. Escrow](./HDC%20v4%20-%20Clase%20#6.pdf)
Programa para intercambiar tokens SPL entre dos usuarios. 'Red p2p': verifica que los usuarios que intervienen lo ejecutan de forma correcta.
Primer usuario -> usuario inicilizador, crea una propuesta para intercambiar tokens A por tokens B, que creara una cuenta de garantia, la cual almacenara los tokens A, hasta que otro usuario decida aceptar el cambio.
- La instrucción 'inicializar': crea una cuenta de escrow, transfiere sus tokens A a una cuenta garantia que esta a cargo del programa(PDA) 
- La instrucción 'finalización': usuario aceptante acepta, la instrucción transfiere los tokens B a la cuenta del usuario inicializador, y la instruccion tambien transfiere los tokens A de la cuenta garantia a la cuenta del finalizador.

- El usuario inicializador, debe tener ciertos tokens A. Por lo tanto, debe tener una 'token account asociada (ATA)' que relacione el inicializador y la cuenta mint.
- El usuario aceptante, debe poseer tokens B, por lo tanto debe aver una ATA vinculada al aceptante y al token mint.

- El escrow, debe almacenar datos:
    - usuario que lo inicia
    - tokens que se quieren intercambiar(token A y B)
    - cantidades de cada tokens

- Cuenta de garantia -> PDA, solo el programa tiene acceso, va a ser una cuenta token.
