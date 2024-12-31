# Cifra de César em Rust

Implementação da Cifra de César em Rust para criptografia e descriptografia com chave fixa (3).

## Funcionalidades

*   Criptografia e descriptografia com chave 3.
*   Tratamento de caracteres fora do alfabeto.
*   Conversão para maiúsculas.

## Como funciona

Substitui letras por outras `chave` posições à frente no alfabeto. Ex: `A` com chave 3 vira `D`.

## Código (Pseudo-código)

### Criptografia

```
criptografar(texto_original):
  texto_cifrado = ""
  para cada caractere em texto_original:
    se caractere em ALFABETO:
      indice_cifrado = (indice(caractere) + CHAVE) MOD tamanho(ALFABETO)
      texto_cifrado += ALFABETO[indice_cifrado]
    senão:
        texto_cifrado += ultimo caractere do alfabeto
  retorna texto_cifrado
```

### Descriptografia

```
descriptografar(texto_cifrado):
  texto_original = ""
  para cada caractere em texto_cifrado:
    se caractere em ALFABETO:
      indice_original = (indice(caractere) - CHAVE)
      se indice_original < 0:
         indice_original = tamanho(ALFABETO) - CHAVE + indice(caractere)
      indice_original = indice_original MOD tamanho(ALFABETO)
      texto_original += ALFABETO[indice_original]
    senão:
        texto_original += ultimo caractere do alfabeto
  retorna texto_original
```

## Execução

`cargo run`

## Melhorias

*   Chave definida pelo usuário.
*   Suporte a minúsculas.
*   Testes unitários.
