# KronosTech – Tienda Virtual de Equipos Tecnológicos

<p align="center">
  <img src="documentacion/assets/images/favicon.svg" alt="KronosTech Logo" width="200"/>
</p>

<p align="center">
  <b>Ecommerce moderno para la venta de equipos de cómputo y tecnología, desarrollado con Rust (Axum), SvelteKit y PostgreSQL.</b>
</p>

---

## Tecnologías Principales

<p align="center">
  <img src="https://img.shields.io/badge/Rust-Axum-orange?logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/SvelteKit-frontend-ff3e00?logo=svelte&logoColor=white" />
  <img src="https://img.shields.io/badge/PostgreSQL-database-316192?logo=postgresql&logoColor=white" />
  <img src="https://img.shields.io/badge/TypeScript-enabled-3178c6?logo=typescript&logoColor=white" />
</p>

---

# Tabla de Contenido

* [1. Descripción General](#descripción-general)
* [2. Estructura del Proyecto](#estructura-del-proyecto)
* [3. Tecnologías Utilizadas](#tecnologías-utilizadas)
* [4. Cómo Ejecutar el Proyecto](#cómo-ejecutar-el-proyecto)
  * [4.1 Backend (Rust + Axum)](#backend-rust-axum)
  * [4.2 Frontend (SvelteKit + TypeScript)](#frontend-sveltekit--typescript)
* [5. Documentación Completa](#documentación-completa)

---

# Descripción General

**KronosTech** es una tienda virtual especializada en equipos tecnológicos, diseñada para ser rápida, segura, escalable y moderna.

Este repositorio contiene:

* **Backend:** API REST desarrollada en **Rust con Axum**
* **Frontend:** Aplicación web en **SvelteKit + TypeScript**
* **Base de datos:** **PostgreSQL**
* **Documentación técnica completa:** requerimientos, arquitectura, diseño, etc.

---

# Estructura del Proyecto

```
KronosTech/
│
├── backend/                # API REST en Rust (Axum)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── routes/
│       ├── controllers/
│       ├── services/
│       └── db/
│
├── frontend/               # Frontend SvelteKit + TypeScript
│   ├── src/
│   ├── static/
│   └── svelte.config.js
│
├── documentacion/          # Documentación técnica completa
│   ├── 1-requisitos/
│   ├── 2-diseño/
│   └── 3-implementacion/
│
└── README.md               # Este archivo
```

---

# Tecnologías Utilizadas

### Backend – Rust + Axum

* Axum (framework web)
* Tokio (runtime asíncrono)
* SQLx / Diesel (ORM / query builder)
* JWT
* Middleware de seguridad
* Validación de inputs

### Frontend – SvelteKit + TypeScript

* SvelteKit para SSR/SSG
* TailwindCSS (opcional)
* Formularios y rutas protegidas
* Fetch API desde el backend Rust

### Base de Datos – PostgreSQL

* Tablas optimizadas para catálogo, usuarios y compras
* Triggers, índices y constraints

---

# Cómo Ejecutar el Proyecto

## Backend (Rust + Axum)

### Requisitos previos

* Rust (stable)
* Cargo
* PostgreSQL

### Pasos

```bash
cd backend
cargo build
cargo run
```

Por defecto la API corre en:

```
http://localhost:3001
```

---

## Frontend (SvelteKit + TypeScript)

### Requisitos

* Node.js 20+
* pnpm / npm

### Pasos

```bash
cd frontend
npm install
npm run dev
```

Frontend disponible en:

```
http://localhost:5173
```

---

# Documentación Completa

Toda la documentación está organizada en carpetas:

### Requisitos

* [1. Descripción del Proyecto](./documentacion/1-requisitos/1.1-descripcion.md)
* [2. Requerimientos Funcionales](./documentacion/1-requisitos/1.2-requisitos-funcionales.md)
* [3. Requerimientos No Funcionales](./documentacion/1-requisitos/1.3-requisitos-no-funcionales.md)
* [4. Casos de Uso](./documentacion/1-requisitos/1.4-casos-de-uso.md)

### Diseño del Sistema

* [Arquitectura General](./documentacion/2-diseno/2.1-arquitectura.md)
* [Modelo Conceptual](./documentacion/2-diseno/2.2-diseno-conceptual.md)
* [Modelo Relacional](./documentacion/2-diseno/2.3-modelo-er.md)
* [Mockups UI](./documentacion/2-diseno/2.4-diseno-ui.md)

### Implementación

* [Stack Tecnológico](./documentacion/3-implementacion/3.1-stack-tecnologico.md)
* [Configuración del Entorno](./documentacion/3-implementacion/3.2-configuracion-entorno.md)
* [Implementación de Módulos](./documentacion/3-implementacion/3.3-implementacion-modulos.md)
* [Scripts SQL](./documentacion/3-implementacion/3.4-scripts-sql.md)

---