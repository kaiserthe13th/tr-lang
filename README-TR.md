# ![tr-lang](./img/logo/logo.png)
#### <center>Hızlı ve Kolay</center>
<hr style="width: 50%;">

[![GitHub lisansı](https://img.shields.io/github/license/kaiserthe13th/tr-lang?label=lisans)](https://github.com/kaiserthe13th/tr-lang/blob/master/LICENSE)
[![GitHub issueları](https://img.shields.io/github/issues/kaiserthe13th/tr-lang?label=issuelar)](https://github.com/kaiserthe13th/tr-lang/issues)
[![GitHub yıldızları](https://img.shields.io/github/stars/kaiserthe13th/tr-lang?label=yıldızlar)](https://github.com/kaiserthe13th/tr-lang/stargazers)
![GitHub yayını (tarihe göre en son)](https://img.shields.io/github/v/release/kaiserthe13th/tr-lang?label=son%20github%20yayını)
![GitHub yayını (önyaınlar dahil tarihe göre en son)](https://img.shields.io/github/v/release/kaiserthe13th/tr-lang?include_prereleases&label=son%20github%20önyayını)
![Crates.io](https://img.shields.io/crates/v/tr-lang)
![Visual Studio Marketplace Sürümü](https://img.shields.io/visual-studio-marketplace/v/kaiserthe13th.tr-lang)
![PyPI](https://img.shields.io/pypi/v/tr-lang-py?label=tr-lang-py)

### ❤️ ile 🇹🇷 de yapılmıştır

tr-lang programlama dili sözdizimini Türkçeye yaklaştırmayı amaçlayan bir programlama dilidir.
tr-lang yarı küme(stack), yarı bölgesel bir hafıza yönetim şekline sahip.
tr-lang'ın matematiksel sözdizimi 0.4.0-rc1 sürümünden beri sondan eklemeliden ortaya eklemeli hale gelmiştir. (a b +) -> (a + b)

[İngilizce](README.md) görüntüle

# :triangular_flag_on_post: İçindekiler
- [🏆 Dilin Hangi Parçaları Tamamlandı?](#-dilin-hangi-parçaları-tamamlandı)
- [🚀 İndirme](#-i̇ndirme)
- [💻 Editör Desteği](#-edit%C3%B6r-deste%C4%9Fi)
- [📖 Dokümantasyonu İncele](#-dok%C3%BCmantasyonu-i%CC%87ncele)
- [🤝 Katkıda Bulunma](#-katkıda-bulunma)
- [📜 Teşekkürler](#-teşekkürler)
- [🔮 Gelecek için Planlar](#-gelecek-i%E7in-planlar)

# 🏆 Dilin Hangi Parçaları Tamamlandı?


## ✔️ Dilin tüm parçaları tamamlanmış gözüküyor!

#### ✔️ tr-lang lexer'ı bitmiş gibi gözüküyor [İssue #1](https://github.com/kaiserthe13th/tr-lang/issues/1#issue-1027652152)<br>
#### ✔️ tr-lang parser'ı bitmiş gibi gözüküyor [İssue #2](https://github.com/kaiserthe13th/tr-lang/issues/2#issue-1027660436)<br>
#### ✔️ tr-lang bytecode bitmiş gibi gözüküyor [İssue #3](https://github.com/kaiserthe13th/tr-lang/issues/3#issue-1027661753)<br>
#### ✔️ tr-lang bytecode okuyucu bitmiş gibi gözüküyor [İssue #4](https://github.com/kaiserthe13th/tr-lang/issues/4#issue-1027663331)<br>
#### ✔️ tr-lang çalışma zamanı bitmiş gibi gözüküyor [İssue #5](https://github.com/kaiserthe13th/tr-lang/issues/5#issue-1027665033)<br>

# 🚀 İndirme

## 🪟 Windows

<!-- // Bozuk! Çalışmıyor.
### 📇 tr-lang_Setup.exe
Windows için programın bir indirici var.
Sadece indirip çalıştırın.
Sadece talimatları takip edin ve hazırsınız!
-->

### 📇 Önden Derlenmiş Program
Windows için programın önden derlenmiş hali var.
Sadece indirin ve bitti!
> Not: Bu program hazır olarak PATH ortam değişkeninde olmayacaktır
> bu tüm sistem içerisinde sadece 'tr-lang' yazarak programa erişemiyexeğiniz anlamına gelir
>
> Eğer tr-lang'ı PATH ortam değişkenine eklemek istiyorsanız Ryan Hoffman tarafından yazılmış bu [öğretici makaleye](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/) göz atın

### 📦 Cargo
```console
$ cargo install tr-lang
```

### ⚙️ Kaynaktan Derleme
gereksinimler: `rust, cargo`
```console
$ git clone https://github.com/kaiserthe13th/tr-lang
$ cd tr-lang
$ cargo install --path .
```
> Not: sadece tr-lang'la oynamak istiyorsanız son komutu `cargo build --release` ile değiştirebilirsiniz
> bu durumda dosyanız `target/release/tr-lang`'da bulunacaktır

## 🍎 MacOS

### 🍺 Homebrew
Homebrew kullanarak indirebilirsiniz
```console
$ brew tap kaiserthe13th/tr-lang
$ brew install tr-lang
```

### 📦 Cargo
```console
$ cargo install tr-lang
```

### ⚙️ Kaynaktan Derleme
gereksinimler: `rust, cargo`
```console
$ git clone https://github.com/kaiserthe13th/tr-lang
$ cd tr-lang
$ cargo install --path .
```
> Not: sadece tr-lang'la oynamak istiyorsanız son komutu `cargo build --release` ile değiştirebilirsiniz
> bu durumda dosyanız `target/release/tr-lang`'da bulunacaktır

## 🐧 Linux

### 🗃️ Debian Paketi
Releases bölümünden tr-lang_x.x.x_amd64.deb paketini indirip onu sisteminize dpkg ile indirebilirsiniz

1. Releases bölümünden istediğiniz sürümü (örnek: `tr-lang_<x.x.x>_amd64.deb`) indirin
2. Terminalden
```console
$ dpkg -i tr-lang_<x.x.x>_amd64.deb
```
> Not: Bazı Linux sistemlerinde sadece dosyaya iki veya bir kere tıklamanız yeterli olacaktır.

### 🎩 RPM Package
1. Releases bölümüne gidin ve istediğiniz sürümü (örnek: `tr-lang_<x.x.x>.x86_64.rpm`) indirin
2. Terminalden
```console
$ rpm -i tr-lang_<x.x.x>.x86_64.rpm
```
> Not: Bazı Linux sistemlerinde sadece dosyaya iki veya bir kere tıklamanız yeterli olacaktır.

### 🍺 Homebrew
Homebrew kullanarak indirebilirsiniz
```console
$ brew tap kaiserthe13th/tr-lang
$ brew install tr-lang
```

### 📦 Cargo
```console
$ cargo install tr-lang
```

### ⚙️ Kaynaktan Derleme
gereksinimler: `rust, cargo`
```console
$ git clone https://github.com/kaiserthe13th/tr-lang
$ cd tr-lang
$ cargo install --path .
```
> Not: sadece tr-lang'la oynamak istiyorsanız son komutu `cargo build --release` ile değiştirebilirsiniz
> bu durumda dosyanız `target/release/tr-lang`'da bulunacaktır

# [📖 Dokümantasyonu İncele](https://tr-lang-docs.netlify.app/türkçe)

# 💻 Editör Desteği
|  | VS Code | Sublime | Atom | Vim/Neovim | Emacs |
|---|---|---|---|---|---|
| **Renklendirme** | [tr-lang Eklentisi](https://marketplace.visualstudio.com/items?itemName=kaiserthe13th.tr-lang) İle | Planlanıyor | Hayır | [tr-lang.vim](highlight/editors/vim) İle | Hayır |
| **Snippetlar** | [tr-lang Eklentisi](https://marketplace.visualstudio.com/items?itemName=kaiserthe13th.tr-lang) İle | Hayır | Hayır | Hayır | Hayır |
> Not: Yardım etmekten çekinmeyin!

# 🤝 Katkıda Bulunma
bug raporlamak, yeni özellik tavsiye etmek veya dokümantasyonu güncellemek için [issue takipçisini](https://github.com/kaiserthe13th/tr-lang/issues) kullanın.

özellikler için <span class="tag">`(enhancement | yükseltme)`</span> etiketini, buglar için <span class="tag">`(bug)`</span> etiketini ve dokümantasyon güncellemeleri için <span class="tag">`(documentation | dökümantasyon)`</span> etiketini kullanın

👍 Bugfix PR'lerine açığız!

# 🔮 Gelecek için Planlar
- Rust benzeri struct'lar
- Gerçek Listeler
- Enterpolasyonlu Yazılar
- Paket Yöneticisi (Trileche, Trill, Tren veya Trial adının verilmesi düşünülüyor)

# 📜 Teşekkürler

- stackoverflow.com'dan [Netwave](https://stackoverflow.com/users/1695172/netwave) adlı kullanıcıya parser yapım sürecinde yaşanan bir [bug'ın](https://stackoverflow.com/questions/69635458/pattern-matching-does-not-allow-me-to-change-values/69636181#69636181) çözümündeki yardımından dolayı teşekkürlerimi sunarım.
- stackoverflow.com'dan [Chayim Friedman](https://stackoverflow.com/users/7884305/chayim-friedman) adlı kullanıcıya BilinmeyenTanımlayıcı hatasının yapımında [closure büyüleri](https://stackoverflow.com/questions/70053866/rust-cloning-hashmapstring-object-without-moving-into-closure-solved) hakkında yardımı için teşekkür ederim.
