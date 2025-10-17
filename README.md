# Simples API REST em Rust com Axum

Este é um projeto inicial e prático criado para aprender e explorar os fundamentos de construção de uma **API REST** utilizando a linguagem **Rust** e o framework web **Axum**.

---

## Tecnologias

- **Rust**: Linguagem de programação.
- **Axum**: Framework web minimalista e modular para Rust, baseado no toolkit Tower.
- **Tokio**: Runtime assíncrono para Rust (necessário para Axum).
- **Serde**: Framework de serialização/desserialização (para manipulação de JSON).

---

## 💡 Sobre o Projeto

O projeto implementa uma **API REST básica** para gerenciamento de usuários.  
Os dados dos usuários são mantidos em memória usando um estado compartilhado (`Arc<Mutex<Vec<User>>>`), o que significa que **os dados não são persistentes** e serão perdidos a cada reinício do servidor.

### Funcionalidades Implementadas

| Método HTTP | Rota             | Descrição                  |
|--------------|------------------|-----------------------------|
| **GET**      | `/`              | Rota raiz simples.          |
| **GET**      | `/users`         | Lista todos os usuários.    |
| **POST**     | `/users`         | Cria um novo usuário.       |
| **GET**      | `/users/:id`     | Busca um usuário por ID.    |
| **DELETE**   | `/users/:id`     | Remove um usuário por ID.   |

---

### Testando a API:
Listar usuários
```curl http://localhost:3000/users```

Buscar usuário específico
```curl http://localhost:3000/users/1```

Deletar usuário
```curl -X DELETE http://localhost:3000/users/1```

Criar usuário
```curl -X POST http://localhost:3000/users \  -H "Content-Type: application/json" \  -d '{"name":"Carlos","email":"carlos@example.com"}'```
