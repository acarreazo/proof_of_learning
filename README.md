# Proof of Learning — Solana Program

Proof of Learning es un programa desarrollado en Solana que permite emitir, gestionar y revocar certificados académicos directamente en blockchain.

El objetivo del programa es demostrar cómo implementar un sistema de credenciales educativas utilizando el paradigma **Account-Oriented Programming** de Solana.

Cada certificado emitido se almacena como una cuenta independiente en la blockchain, permitiendo su verificación pública y evitando la falsificación de certificados académicos.

---

# Arquitectura del Programa

El programa sigue el modelo de programación orientado a cuentas de Solana.

Program  
│  
▼  
Course PDA  
│  
▼  
Certificate PDA  
Certificate PDA  
Certificate PDA  

Un **Course** actúa como un registro (registry) de certificados emitidos.

Cada **Certificate** es una cuenta independiente asociada a un estudiante.

---

# Modelo de Datos

## Course

Representa un curso académico que puede emitir certificados.

Campos:

| Campo | Tipo | Descripción |
|------|------|-------------|
instructor | Pubkey | Wallet del instructor |
nombre | String | Nombre del curso |
certificados | Vec<Pubkey> | Lista de certificados emitidos |

---

## Certificate

Representa un certificado académico emitido a un estudiante.

Campos:

| Campo | Tipo | Descripción |
|------|------|-------------|
student | Pubkey | Wallet del estudiante |
course | Pubkey | Cuenta del curso |
instructor | Pubkey | Instructor que emitió el certificado |
titulo | String | Nombre del certificado |
issued_at | i64 | Fecha de emisión |
activo | bool | Estado del certificado |

---

# PDAs (Program Derived Addresses)

El programa utiliza PDAs para generar direcciones determinísticas controladas por el programa.

Esto permite garantizar unicidad y seguridad sin necesidad de claves privadas.

---

## Course PDA

Se genera usando las siguientes seeds:

```
["course", nombre_curso, instructor]
```

Conceptualmente:

```
Course PDA = hash("course", nombreCurso, instructor)
```

Esto garantiza que cada curso tenga una dirección única.

---

## Certificate PDA

Se genera usando las siguientes seeds:

```
["certificate", student, course]
```

Conceptualmente:

```
Certificate PDA = hash("certificate", student, course)
```

Esto garantiza que:

1 estudiante  
1 curso  
1 certificado  

---

# Instrucciones del Programa

El programa implementa operaciones CRUD sobre certificados académicos.

---

## Crear Curso

```
crear_curso(nombre)
```

Crea una cuenta **Course**.

Solo el instructor que crea el curso puede gestionarlo.

---

## Emitir Certificado (Create)

```
emitir_certificado(titulo)
```

Crea una cuenta **Certificate** asociada a:

- estudiante
- curso
- instructor

Además agrega la dirección del certificado al vector `certificados` del curso.

---

## Actualizar Certificado (Update)

```
actualizar_certificado(nuevo_titulo)
```

Permite modificar la información del certificado.

Solo el instructor del curso puede realizar esta operación.

---

## Eliminar Certificado (Delete)

```
eliminar_certificado()
```

Elimina un certificado cerrando la cuenta correspondiente.

También elimina la referencia del certificado en el vector de certificados del curso.

---

## Leer Certificados (Read)

La lectura de datos no se realiza dentro del programa.

El cliente obtiene la información usando:

```
program.account.course.fetch(...)
program.account.certificate.fetch(...)
```

Esto es más eficiente en Solana.

---

# Patrones de Diseño Utilizados

El programa implementa varios patrones comunes en desarrollo de programas de Solana.

---

## PDA Pattern

Se utilizan PDAs para generar direcciones determinísticas para:

- cursos
- certificados

---

## Authority Pattern

El instructor del curso controla:

- actualización de certificados
- eliminación de certificados

---

## Registry Pattern

La cuenta **Course** funciona como un registro que mantiene referencias a todos los certificados emitidos.

---

## Account Composition

Un curso contiene referencias a múltiples certificados.

Course  
│  
├─ Certificate  
├─ Certificate  
└─ Certificate  

---

# Flujo de Uso

1. El instructor crea un curso.

```
crear_curso("Solana Bootcamp")
```

2. Se emite un certificado a un estudiante.

```
emitir_certificado("Solana Developer Certificate")
```

3. El certificado queda registrado en la blockchain.

4. Cualquier usuario puede verificar el certificado leyendo la cuenta correspondiente.

---

# Beneficios del Enfoque

Este diseño permite:

- credenciales académicas verificables
- eliminación de falsificación de certificados
- transparencia en la emisión de certificados
- verificación pública en blockchain

---

# Tecnologías Utilizadas

- Solana
- Anchor Framework
- Rust
- Solana Playground

---

# Autor
Anllelo Carreazo

Proyecto desarrollado como ejercicio educativo para demostrar el modelo de programación orientado a cuentas de Solana y el uso de PDAs en la construcción de aplicaciones descentralizadas.
