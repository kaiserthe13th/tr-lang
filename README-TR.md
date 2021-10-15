# tr-lang

tr-lang programlama dili sözdizimini Türkçeye yaklaştırmayı amaçlayan bir programlama dilidir.
tr-lang küme(stack) orantılı bir dil ve matematik için ters leh notasyonu kullanıyor.

# Dilin hangi parçaları tamamlandı?

#### ❌ tr-lang lexer'ı çalışma altında ![#1](https://github.com/kaiserthe13th/tr-lang/issues/1#issue-1027652152)<br>
#### ❌ tr-lang parser'ı tamamlanmadı<br>
#### ❌ tr-lang bytecode tamamlanmadı<br>
#### ❌ tr-lang bytecode okuyucu tamamlanmadı<br>
#### ❌ tr-lang çalışma zamanı tamamlanmadı<br>

# Dil Spesifikasyonu

#### ❕ Aşağıdakilerin hiçbiri tamamlanmamıştır

## Veri Tipleri

### Yazı

```py
"Bu bir yazı\n"
'Merhaba, Dünya!\n'
```

### Sayı

```py
10328
142.32
```

### Boolean

```py
doğru
yanlış
```
## Anahtar Kelimeler

### de
verş tiplerini ekrana yazmak için kullanılır
```py
"Merhaba, Dünya!\n" de
1232 de ' ' de
doğru de
```
#### Stdout
```stdout
Merhaba, Dünya!
1232 doğru
```

### ise
tr-lang'da if ifadesi
koşulları kontrol etmek için kullanılır
```py
10 0 > ise
  "Merhaba\n" de
son
```

### yoksa
tr-lang'da else ifadesi
kontrol edilen koşullar geçersizse birşeyler yap
```py
10 0 < ise
  "Buraya nasıl geldik?\n" de
yoksa
  "Evren hala çalışıyor!\n" de
son
```

### son
ise-yoksa bloğunu bitirir

### Planlanmamış...
