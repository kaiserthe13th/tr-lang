# ![tr-lang](./img/logo/logo.png)
#### <center>Hızlı ve Kolay</center>
<hr style="width: 50%;">

### ❤️ ile 🇹🇷 de yapılmıştır


tr-lang programlama dili sözdizimini Türkçeye yaklaştırmayı amaçlayan bir programlama dilidir.
tr-lang küme(stack) orantılı bir dil ve matematik için ters leh notasyonu kullanıyor.

# İçindekiler
- [Dilin Hangi Parçaları Tamamlandı?](#dilin-hangi-parçaları-tamamlandı)
- [İndirme](#indirme)
- [Vikiye Bak](#vikiye-bak)
- [Katkıda Bulunma](#katkıda-bulunma)
- [Teşekkürler](#teşekkürler)

# Dilin Hangi Parçaları Tamamlandı?


## ✔️ Dilin tüm parçaları tamamlanmış gözüküyor!

#### ✔️ tr-lang lexer'ı bitmiş gibi gözüküyor [İssue #1](https://github.com/kaiserthe13th/tr-lang/issues/1#issue-1027652152)<br>
#### ✔️ tr-lang parser'ı bitmiş gibi gözüküyor [İssue #2](https://github.com/kaiserthe13th/tr-lang/issues/2#issue-1027660436)<br>
#### ✔️ tr-lang bytecode bitmiş gibi gözüküyor [İssue #3](https://github.com/kaiserthe13th/tr-lang/issues/3#issue-1027661753)<br>
#### ✔️ tr-lang bytecode okuyucu bitmiş gibi gözüküyor [İssue #4](https://github.com/kaiserthe13th/tr-lang/issues/4#issue-1027663331)<br>
#### ✔️ tr-lang çalışma zamanı bitmiş gibi gözüküyor [İssue #5](https://github.com/kaiserthe13th/tr-lang/issues/5#issue-1027665033)<br>

# İndirme

## Homebrew
Homebrew kullanarak indirebilirsiniz
```sh
brew tap kaiserthe13th/tr-lang
brew install tr-lang
```

## Cargo
```sh
cargo install tr-lang
```

## Kaynaktan Derleme
gereksinimler: `rust, cargo`
```sh
git clone https://github.com/kaiserthe13th/tr-lang
cd tr-lang
cargo build --release
```
dosyanız `target/release/tr-lang`'da bulunacaktır

# [Vikiye Bak](https://github.com/kaiserthe13th/tr-lang/wiki/Anasayfa---TR)
- [Dil Spesifikasyonu](https://github.com/kaiserthe13th/tr-lang/wiki/Dil-Spesifikasyonu-TR)

# Katkıda Bulunma
bug raporlamak, yeni özellik tavsiye etmek veya dokümantasyonu güncellemek için [issue takipçisini](https://github.com/kaiserthe13th/tr-lang/issues) kullanın.

özellikler için <span class="tag">`(enhancement | yükseltme)`</span> etiketini, buglar için <span class="tag">`(bug)`</span> etiketini ve dokümantasyon güncellemeleri için <span class="tag">`(documentation | dökümantasyon)`</span> etiketini kullanın

:+1: Bugfix PR'lerine açığız!

# Teşekkürler

stackoverflow.com'dan [Netwave](https://stackoverflow.com/users/1695172/netwave) adlı kullanıcıya parser yapım sürecinde yaşanan bir [bug'ın](https://stackoverflow.com/questions/69635458/pattern-matching-does-not-allow-me-to-change-values/69636181#69636181) çözümündeki yardımından dolayı teşekkürlerimi sunarım.

