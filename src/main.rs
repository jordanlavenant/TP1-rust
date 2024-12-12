fn main() {
    #[derive(Debug, Clone)]
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
    }

    #[derive(Debug, Clone)]
    struct Student {
        nom: String,
        prenom: String,
        filiere: Filiere,
    }

    let s = Student {
        nom: String::from("Doe"),
        prenom: String::from("John"),
        filiere: Filiere::Info,
    };

    fn affichage_filiere(f: Filiere) {
        f.appreciation();
    }

    println!("Affichage : {:?}", s);

    let mut copie = s.clone();
    copie.nom = String::from("Doe Jr.");
    println!("Affichage : {:?}", copie);
    println!("Affichage : {:?}", s);

    let f = Filiere::Chimie;

    affichage_filiere(f);
    affichage_filiere(f);
}
