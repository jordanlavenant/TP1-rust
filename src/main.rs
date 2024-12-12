fn main() {
    #[derive(Debug, Clone, Copy)]
    enum Filiere {
        Chimie,
        GEA,
        GMP,
        GTE,
        Info,
        QLIO,
    }

    impl Filiere {
        fn appreciation(self) {
            match self {
                Filiere::Chimie => println!("Chimie : C'est la chimie"),
                Filiere::GEA => println!("GEA : C'est la gestion"),
                Filiere::GMP => println!("GMP : C'est la mécanique"),
                Filiere::GTE => println!("GTE : C'est l'électrotechnique"),
                Filiere::Info => println!("Info : C'est l'informatique"),
                Filiere::QLIO => println!("QLIO : C'est la qualité"),
            }
        }

        fn affichage_filiere(self) {
            self.appreciation();
        }
    }

    #[derive(Debug, Clone)]
    struct Student {
        nom: String,
        prenom: String,
        filiere: Filiere,
    }

    impl Student {
        fn affichage_etu(&self) {
            println!("Nom : {}", self.nom);
            println!("Prénom : {}", self.prenom);
            self.filiere.appreciation();
        }

        fn japanifie(&mut self) {
            self.nom = String::from("Doe-San");
            self.prenom = String::from("John-Kun");
        }
    }

    let mut s = Student {
        nom: String::from("Doe"),
        prenom: String::from("John"),
        filiere: Filiere::Info,
    };

    println!("Affichage : {:?}", s);

    let mut copie = s.clone();
    copie.nom = String::from("Doe Jr.");
    println!("Affichage : {:?}", copie);
    println!("Affichage : {:?}", s);

    let f = Filiere::Chimie;
    f.affichage_filiere();
    f.affichage_filiere();

    let s0 = String::from("Hello");
    let mut s1 = s0.clone();

    s1 = String::from("World");

    println!("s0 = {}", s0);
    println!("s1 = {}", s1);

    s.affichage_etu();
    s.japanifie();
    s.affichage_etu();
}
