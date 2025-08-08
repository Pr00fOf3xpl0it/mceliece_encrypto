
<img width="1536" height="1024" alt="ChatGPT Image 7 ago 2025, 09_16_53 p m" src="https://github.com/user-attachments/assets/0cf5d808-6f74-48b0-b56c-2222d56cfd24" />

# 🔐 McEliece Encryptor CLI

**Cifrado local con McEliece + AES-GCM + Firma Post-Cuántica (Dilithium2)**  
Autor: Mauro Carrillo — HackSyndicate 2025

---

## ¿Qué es esto?

Una herramienta CLI que combina cifrado y firma digital **post-cuántica** para proteger archivos.  
Usa dos algoritmos reconocidos por el NIST para la era post-cuántica:

- 🔒 **McEliece**: para cifrado asimétrico (muy rápido, resistente a ataques cuánticos).
- ✍️ **Dilithium2**: para firma digital (verifica integridad y autenticidad de archivos).

---

## Comandos disponibles

| Comando    | Descripción |
|------------|-------------|
| `keygen`   | Genera claves pública/privada para cifrado (McEliece) |
| `encrypt`  | Cifra un archivo usando clave pública (.pub) |
| `decrypt`  | Descifra un archivo cifrado con clave privada (.priv) |
| `sign`     | Firma un archivo con Dilithium2 y genera claves de firma |
| `verify`   | Verifica la firma de un archivo con la clave pública (.pk) |

---

## Flujo de Cifrado

```
┌────────────┐       ┌──────────────┐       ┌────────────┐
│  keygen    │       │   encrypt    │       │   decrypt  │
└────┬───────┘       └────┬─────────┘       └────┬───────┘
     │                    │                       │
     ▼                    ▼                       ▼
 clave.pub        +-> archivo.enc <-+        clave.priv
 clave.priv         (AES-GCM + McEliece)
```

- Se genera una clave AES aleatoria.
- Se cifra el archivo con AES-GCM.
- La clave AES se cifra con McEliece (`clave.pub`).
- Todo se guarda en `.enc`.

---

## Flujo de Firma

```
┌────────────┐      ┌────────────┐      ┌────────────┐
│   sign     │      │   verify   │      │ Resultado  │
└────┬───────┘      └────┬───────┘      └────┬───────┘
     │                   │                   ▼
     ▼                   ▼              Firma válida ❯ ✅
 archivo + sk      archivo + sig + pk
 (Dilithium2)
```

- Se generan claves (`.sk`, `.pk`) de firma post-cuántica (Dilithium2).
- Se firma el archivo con `.sk` → `.sig`.
- Se puede verificar con `.pk` y `.sig`.

---

## 📦 Archivos generados

| Archivo         | Descripción |
|------------------|-------------|
| `*.pub`          | Clave pública McEliece (cifrado) |
| `*.priv`         | Clave privada McEliece (descifrado) |
| `*.pk`           | Clave pública Dilithium2 (firma) |
| `*.sk`           | Clave privada Dilithium2 (firma) |
| `*.sig`          | Firma del archivo |
| `*.enc`          | Archivo cifrado completo |

---

## 🚀 Ejemplo de uso

```bash
# Generar claves de cifrado (McEliece)
./mceliece_encryptor keygen -o claves/miclave

# Cifrar un archivo
./mceliece_encryptor encrypt -f secreto.pdf -p claves/miclave.pub

# Descifrar un archivo
./mceliece_encryptor decrypt -f secreto.pdf.enc -r claves/miclave.priv

# Firmar un archivo (Dilithium2)
./mceliece_encryptor sign -f secreto.pdf -o firmas/mifirma

# Verificar la firma
./mceliece_encryptor verify -f secreto.pdf -s firmas/mifirma.sig -p firmas/mifirma.pk
```

---

## Sobre los algoritmos

- **McEliece**: algoritmo de cifrado basado en códigos correctores de errores (Goppa Codes). Seleccionado por el NIST como candidato para resistir computadoras cuánticas. Extremadamente rápido en cifrado y resistente a ataques estructurales.
- **Dilithium2**: esquema de firma digital basado en retículas (lattices). Forma parte del conjunto CRYSTALS de NIST PQC y es eficiente y seguro contra ataques cuánticos.

---

## 🧮 ¿Pero qué es realmente una “firma” en Dilithium2?

Cuando decimos que firmamos un archivo, **no hablamos de una “firma textual” o de un hash firmado**, como en RSA o ECDSA. En Dilithium2, **una firma digital es un conjunto de vectores calculados** sobre estructuras matemáticas llamadas **retículas (lattices)**.

Estos vectores permiten comprobar que un mensaje fue generado por alguien que conoce la clave privada, pero sin revelar dicha clave.

### 🔐 Firma ≠ texto, Firma = cálculo de vectores

Dilithium2 se basa en problemas como **SIS (Short Integer Solution)** y **LWE (Learning With Errors)**. No hay curvas ni factorizaciones; solo álgebra modular.

---

## 📐 Fundamentos matemáticos de Dilithium2 (resumen técnico)

### 🗝️ Claves

- Clave pública:

```
A ∈ ℤ_q^{k × l}, t = A · s + e
```

Donde:
- `A`: matriz generada de forma determinista (a partir de una semilla)
- `s`, `e`: vectores secretos de baja norma
- `t`: parte de la clave pública

### 📝 Proceso de firma

Dado un mensaje µ:

1. Generar un vector aleatorio y ∈ ℤ_q^l
2. Calcular: w = A · y
3. Derivar un reto: c = H(µ, w₁)
4. Calcular la firma: z = y + c · s

La firma es el par (z, c)

### ✅ Verificación

1. Reconstruir: w' = A · z − c · t
2. Verificar: ¿c =? H(µ, w'₁)

---

## Seguridad

- ✔️ Todas las claves y firmas son post-cuánticas (McEliece + Dilithium2).
- ✔️ El contenido del archivo se cifra con AES-GCM (autenticado).
- ❌ No se requiere conexión a internet. Todo se hace local.

---

## Créditos

Desarrollado por [**HackSyndicate**](https://www.hacksyndicate.xyz)  
Coordinado por [**Mauro Carrillo (Pr00f)**](https://www.linkedin.com/in/mauro-carrillo-7a326a208)
