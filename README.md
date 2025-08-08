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

```text
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

```text
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

## Seguridad

- ✔️ Todas las claves y firmas son post-cuánticas (McEliece + Dilithium2).
- ✔️ El contenido del archivo se cifra con AES-GCM (autenticado).
- ❌ No se requiere conexión a internet. Todo se hace local.

---

## Créditos

Desarrollado por [**HackSyndicate**](https://www.hacksyndicate.xyz)  
Coordinado por [**Mauro Carrillo (Pr00f)**](https://www.linkedin.com/in/mauro-carrillo-7a326a208)
