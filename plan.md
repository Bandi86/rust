# Rust

# 1. Alapok (2-3 hét)

Telepítés és eszközök: Rust telepítése, Cargo csomagkezelő megismerése
Alapvető szintaxis: változók, típusok, függvények, vezérlési struktúrák
Tulajdonjog (ownership) rendszer: Rust egyik legfontosabb koncepciója
Ajánlott források:

The Rust Programming Language hivatalos könyv
Rust by Example
Rustlings gyakorlatok

Mini projekt: Egyszerű parancssori alkalmazás (pl. fájlkezelő vagy todo lista)

# 2. Középhaladó koncepciók (3-4 hét)

Struktúrák és tulajdonságok (traits): objektum-orientált programozás Rust módra
Hibakezélés: Result és Option típusok használata
Modulok és csomagok: kód szervezése
Generikus típusok és életciklusok (lifetimes)

Projekt: CLI eszköz fejlesztése, ami valamilyen webes API-hoz kapcsolódik (hasonlóan a Node.js-ben készített eszközökhöz)

# 3. Haladó témák (4-6 hét)

Aszinkron programozás: async/await, tokio framework
Párhuzamos programozás: szálak, mutexek, atomi változók
Unsafe Rust: alacsony szintű műveletek
FFI (Foreign Function Interface): más nyelvű kódok integrálása

Projekt: Webes háttéralkalmazás fejlesztése (REST API)

# 4. Web fejlesztés Rust-ban (4-6 hét)

Mivel web fejlesztői háttérrel rendelkezik, ez különösen érdekes lehet:

Web szerverek: Actix-web, Rocket vagy Axum keretrendszerek
WebAssembly: Rust kód futtatása böngészőben

Yew vagy Leptos (React-szerű keretrendszerek)


Adatbázis integráció: SQLx, Diesel ORM

Nagyprojekt: Fullstack alkalmazás Rust backenddel és WebAssembly frontenddel

# 5. Rendszerprogramozás és performancia (opcionális, 3-4 hét)

Rendszerprogramozás alapjai
Memóriaoptimalizálás
Beágyazott rendszerek programozása

Projekt: Kis teljesítményű eszköz alkalmazása vagy rendszerszintű eszköz
Kihívást jelentő záró projekt
Már a meglévő JavaScript/web ismereteit és új Rust tudását kombinálva:
Egy hibrid alkalmazás fejlesztése, amely:

Rust backendet használ nagy teljesítményű számításokhoz vagy adatfeldolgozáshoz
React/Next.js frontendet a felhasználói felülethez
WebAssembly modult teljesítményigényes kliens-oldali műveletekhez

Például: valós idejű adatelemző alkalmazás, játék, kollaboratív szerkesztő, stb.
Tanulási tippek

Kódoljon rendszeresen - napi 1-2 óra jobb, mint heti 10 óra egyben
Használja a közösségi forrásokat - r/rust, Rust Discord, Stack Overflow
Olvassa mások kódját - népszerű Rust projektek tanulmányozása
Lépésről lépésre haladjon - a tulajdonjog rendszere eleinte nehéz lehet