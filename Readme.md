# Description
Création de Csv

# Usage
1. Implémenter le trait CsvBuilder au niveau de la structure à sérializer
- Si le CSV est avec en-tête à internationaliser :  
(*Dans ce cas, on suppose que le fichier contenant les traductions est au format toml.*)  
Utiliser `Cell::new_title`, avec le paramètre `title` contenant la clé à traduire dans le fichier toml. Si la clé se trouve dans une section, il suffit de séparer le nom de la section et de la clé par un point
fr.toml :
```
    [[commons]]
    name="Nom"
```

csv.rs :  
```
impl CellsBuilder for MyStruct {
    fn get_cells(&self) -> Vec<Cell> {
        vec![
            Cell::new_title("commons.name", self.name.clone())
        ]
    }
}

```
- Si le CSV est à en-tête fixe :  
Utiliser dans `title` le label de la colonne CSV

2. Sérialisation  
- Si le CSV est avec en-tête à internationaliser :
```
let csv = CSv::new...
csv.serialize_i18n_toml(<vec de MyStruct>, <toml::Table de tractuction>)
```
- Si le CSV est à en-tête fixe :
```
let csv = CSv::new...
csv.serialize(<vec de MyStruct>)
```
 

---
# Versions
12/11/24 - v1.1.0  
Modification nom trait: CsvStruct -> CellsBuilder  
Renommage function: Csv::serialize_i18n -> serialize_i18n_toml

08/11/24 - v1.0.0  
Version initiale
