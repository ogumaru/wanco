# wanco

wanco stands for `WAsm text eNCOder`.

## これなに / About

ブラウザ上で文字を別のエンコーディングに変換するやつ。

The one converting strings to other text encoding.

### サポートしているエンコーディング / Supported Encoding

- UTF-8
- Shift_JIS
- EUC-JP

## コンセプト / Concept

CSV の作成など、Web ブラウザ上のみで巨大な UTF-8 形式の文字列を Shift_JIS に変換することを想定。

This assumed to converting huge UTF-8 format strings to Shift_JIS in the Web browser alone, e.g., for creating CSV files.

- [encoding.js](https://github.com/polygonplanet/encoding.js/blob/master/README_ja.md)
- [Encoding API](https://developer.mozilla.org/en-US/docs/Web/API/Encoding_API)

との比較は下記

Comparation bellow

|                |         encoding.js          |         Encoding API         |         wanco🐶          |
| -------------- | :--------------------------: | :--------------------------: | :----------------------: |
| Implementation |          JavaScript          |            Native            |       WebAssembly        |
| Encode To      | UTF-8, Shift_JIS, EUC-JP ... |            UTF-8             | UTF-8, Shift_JIS, EUC-JP |
| Decode From    | UTF-8, Shift_JIS, EUC-JP ... | UTF-8, Shift_JIS, EUC-JP ... |            ー            |

## インストール / Install

```bash
npm install --save wanco
```

## 使い方 / How to Use

```typescript
import init, { encode } from "wanco";
init().then(() => {
  const encoded = encode(content, "shift_jis");
  // Encoded bytes.(number[])
  console.log(encoded.bytes);
  // Text encoding name.(string)
  console.log(encoded.encoding);
  // Whether having any unmappable characters.(bool)
  console.log(encoded.has_unmappable);
});
```

### 例 / Example

```typescript
import init, { encode } from "wanco";
export const downloadAsCSV = (content: string): void => {
  init().then(() => {
    // Specify text encoding.
    const encoded = encode(content, "shift_jis");
    // "bytes" property has encoded codes.
    // Use new Uint8Array() to save through Blob.
    const data = new Uint8Array(encoded.bytes);
    const blob = new Blob([data], { type: "text/csv" });
    const anchor = document.createElement("a");
    const blobURL = URL.createObjectURL(blob);
    anchor.href = blobURL;
    anchor.click();
    URL.revokeObjectURL(blobURL);
  });
};
```

## 開発 / Development

wanco は`wasm-pack`を利用して作成されています。

wanco is build using `wasm-pack`.

### ローカルでの実行 / Run locally

```bash
wasm-pack build --target web
python3 -m http.server 8080
```

`http://localhost:8080/`へアクセスし、デモページである`index.html`を開く

Access `http://localhost:8080/` to open `index.html`. (demo page file)

### ビルド / Build

```bash
wasm-pack build --target web
```

### パッケージ / Packaging

```bash
wasm-pack pack
```

### テスト / Test

```bash
wasm-pack test --firefox --headless
```
