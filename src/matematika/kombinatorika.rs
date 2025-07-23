#![allow(non_snake_case)]

/// Fungsi faktorial
/// Factorial function
pub fn faktorial(n: u64) -> u64 
{
    (1..=n).product()
}

/// Fungsi kombinasi (tanpa pengulangan)
/// Combination function (without repetition)
pub fn kombinasi(n: u64, k: u64) -> u64 
{
    if k > n { return 0; }
    faktorial(n) / (faktorial(k) * faktorial(n - k))
}

/// Fungsi permutasi (tanpa pengulangan)
/// Permutation function (without repetition)
/// P(n, r) = n! / (n - r)!
pub fn permutasi(n: u64, r: u64) -> u64 
{
    if r > n {
        panic!("r tidak boleh lebih besar dari n"); // r must not be greater than n
    }
    faktorial(n) / faktorial(n - r)
}

/// Kombinasi dengan pengulangan
/// Combination with repetition
/// C'(n, r) = (n + r - 1)! / (r! * (n - 1)!)
pub fn kombinasi_perulangan(n: u64, r: u64) -> u64 
{
    faktorial(n + r - 1) / (faktorial(r) * faktorial(n - 1))
}

/// Permutasi dengan pengulangan
/// Permutation with repetition
/// P(n; n1, n2, ..., nk) = n! / (n1! * n2! * ... * nk!)
pub fn permutasi_perulangan(n: u64, pengulangan: &[u64]) -> u64 
{
    let pembilang = faktorial(n);
    let penyebut: u64 = pengulangan.iter().map(|&x| faktorial(x)).product();
    pembilang / penyebut
}

/// Modul Peluang
/// Probability module
pub mod Peluang {
    #[derive(Debug)]
    pub struct Dadu {
        pub angka: [i64; 6],
    }

    impl Dadu {
        /// Membuat objek Dadu baru
        /// Create a new Dice object
        pub fn new() -> Self {
            Self { angka: [1, 2, 3, 4, 5, 6] }
        }

        /// Peluang muncul angka tertentu
        /// Probability of a specific number appearing
        pub fn muncul_angka(&self, target: i64) -> f64 {
            if self.angka.contains(&target) {
                1.0 / self.angka.len() as f64
            } else { 0.0 }
        }

        /// Peluang muncul angka genap
        /// Probability of an even number appearing
        pub fn muncul_genap(&self) -> f64 {
            let count = self.angka.iter().filter(|&&x| x % 2 == 0).count();
            count as f64 / self.angka.len() as f64
        }

        /// Peluang muncul angka lebih dari batas
        /// Probability of a number greater than limit appearing
        pub fn muncul_lebih_dari(&self, batas: i64) -> f64 {
            let count = self.angka.iter().filter(|&&x| x > batas).count();
            count as f64 / self.angka.len() as f64
        }

        /// Peluang muncul angka kurang dari batas
        /// Probability of a number less than limit appearing
        pub fn muncul_kurang_dari(&self, batas: i64) -> f64 {
            let count = self.angka.iter().filter(|&&x| x < batas).count();
            count as f64 / self.angka.len() as f64
        }
    }

    #[derive(Debug)]
    pub struct Koin {
        pub sisi: [char; 2], // 'A' = Angka, 'G' = Gambar // 'A' = Heads, 'G' = Tails
    }

    impl Koin {
        /// Membuat objek Koin baru
        /// Create a new Coin object
        pub fn new() -> Self {
            Self { sisi: ['A', 'G'] }
        }

        /// Peluang muncul sisi tertentu
        /// Probability of a specific side appearing
        pub fn muncul(&self, target: char) -> f64 {
            if self.sisi.contains(&target) {
                1.0 / self.sisi.len() as f64
            } else { 0.0 }
        }

        /// Peluang muncul sisi beruntun
        /// Probability of a side appearing consecutively
        pub fn muncul_beruntun(&self, target: char, jumlah: u32) -> f64 {
            if self.sisi.contains(&target) {
                (1.0 / self.sisi.len() as f64).powi(jumlah as i32)
            } else { 0.0 }
        }

        /// Peluang muncul setidaknya satu sisi tertentu dalam beberapa kali lempar
        /// Probability of at least one specific side appearing in several tosses
        pub fn muncul_setidaknya_satu(&self, target: char, jumlah: u32) -> f64 {
            if self.sisi.contains(&target) {
                let lawan = match target {
                    'A' => 'G',
                    'G' => 'A',
                    _ => return 0.0,
                };
                1.0 - self.muncul_beruntun(lawan, jumlah)
            } else { 0.0 }
        }
    }

    #[derive(Debug)]
    pub struct KantongKelereng {
        pub merah: u32,
        pub putih: u32,
    }

    impl KantongKelereng {
        /// Membuat objek kantong kelereng baru
        /// Create a new marble bag object
        pub fn new(merah: u32, putih: u32) -> Self {
            Self { merah, putih }
        }

        /// Peluang mengambil satu kelereng dengan warna tertentu
        /// Probability of picking one marble of a specific color
        pub fn muncul_satu(&self, warna: char) -> f64 {
            let total = self.merah + self.putih;
            match warna {
                'M' => self.merah as f64 / total as f64, // 'M' = Merah (Red)
                'P' => self.putih as f64 / total as f64, // 'P' = Putih (White)
                _ => 0.0, // warna tidak valid // invalid color
            }
        }

        /// Peluang mengambil dua kelereng berurutan dengan warna tertentu
        /// Probability of picking two marbles in a row with specific colors
        pub fn muncul_dua_berurutan(&self, warna_pertama: char, warna_kedua: char) -> f64 {
            let total = self.merah + self.putih;

            let (jumlah_pertama, jumlah_kedua) = match (warna_pertama, warna_kedua) {
                ('M', 'M') => (self.merah, self.merah - 1),
                ('M', 'P') => (self.merah, self.putih),
                ('P', 'M') => (self.putih, self.merah),
                ('P', 'P') => (self.putih, self.putih - 1),
                _ => return 0.0, // input tidak valid // invalid input
            };

            (jumlah_pertama as f64 / total as f64) * (jumlah_kedua as f64 / (total - 1) as f64)
        }
    }
}
