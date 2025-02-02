# ejercicios
## bootcamp
### 1. Calculando la renta
- Si una cuenta en Solana almacena un PublicKey, un saldo de tipo u64, un timestamp de tipo u64 y un estado de tipo bool ¿cuál es el tamaño total en bytes de la cuenta?
- Si una cuenta en Solana almacena un PublicKey, un entero de tipo u32 y un flotante de tipo f64, ¿cuál es el mínimo de $SOL que debe tener para estar exenta de pagar renta?
- Si quisieras crear una cuenta en Solana que almacene 0 bytes, ¿cuál es el mínimo de $SOL que debe tener para estar exenta de pagar renta?

<details>
<summary>Resolución ❓</summary>

- Calculo de el tamaño total en bytes:

    - PublicKey = 32 bytes
    - u64 (saldo) = 8 bytes
    - u64 (timestamp) = 8 bytes
    - bool (estado) = 1 byte
    - Total = 32 + 8 + 8 + 1 = 49 bytes

- Mínimo de $SOL para `rent exception`:
    - Primero calculamos el tamaño:
        * PublicKey = 32 bytes
        * u32 = 4 bytes
        * f64 = 8 bytes
        * overhead = 128 bytes
        * Total = 32 + 4 + 8 = 44 bytes + 128 bytes = 172 bytes
    - La fórmula para calcular SOL exento de renta es:
        * (tamaño_cuenta * 0.00000348 SOL * 2 años)
        - Por lo tanto: 172 * 0.00000348 * 2 = 0.00119712 SOL

- Mínimo de $SOL para `rent exception`, cuenta 0 bytes:
    - Todas las cuentas en Solana tienen un overhead mínimo de 128 bytes
    - Por lo tanto, incluso con 0 bytes de datos, necesitamos calcular:
    * 128 * 0.00000348 * 2 = 0.00089088 SOL

</details>

### 2. Definiendo cuentas e intrucciones
- Crea un nuevo proyecto Anchor en Solana Playground y define la estructura de datos de una cuenta que contiene un mensaje de máximo 150 caracteres. 
- En el mismo proyecto, define el contexto y la función para una instrucción que permita crear una nueva cuenta con la estructura definida anteriormente. 
- En el mismo proyecto, define el contexto y la función para la instrucción que permita modificar los datos de una cuenta existente. 
<details><summary>Resolución</summary>

</details>

### x. Definiendo cuentas e intrucciones
- Crea un nuevo proyecto Anchor en Solana Playground y define la estructura de datos de una cuenta que contiene un mensaje de máximo 150 caracteres. 
- En el mismo proyecto, define el contexto y la función para una instrucción que permita crear una nueva cuenta con la estructura definida anteriormente. 
- En el mismo proyecto, define el contexto y la función para la instrucción que permita modificar los datos de una cuenta existente. 
<details><summary>Resolución</summary>

</details>