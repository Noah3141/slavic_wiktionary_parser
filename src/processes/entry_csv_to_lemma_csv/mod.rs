use std::{
    fs::File,
    io::{BufWriter, Read, Write},
};

use wiktionary_parser::models::language::Language;

use crate::traits::accented::Accented;

pub fn entry_csv_to_lemma_csv(
    entry_csv: &str,
    out: &str,
    overwrite: bool,
    language: Language,
) -> Result<(), ()> {
    let out_file = match overwrite {
        true => File::options()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(out)
            .expect("creation of out file with overwrite"),
        false => File::options()
            .read(true)
            .append(true)
            .truncate(false)
            .create_new(true)
            .open(out)
            .expect("creation of out file with !overwrite"),
    };

    let mut writer = BufWriter::with_capacity(1024 * 6, out_file);

    let mut csv = File::options()
        .read(true)
        .write(false)
        .append(false)
        .create(false)
        .open(entry_csv)
        .expect("opening the entry csv file");

    let mut csv_file_str = String::with_capacity(1024 * 6);
    csv.read_to_string(&mut csv_file_str)
        .expect("reading to string");

    let rows = csv_file_str.lines();

    println!("{} rows...", rows.clone().count());

    match language {
        Language::Russian => {
            for row in rows {
                let mut cols = row.split("|");
                let lemma = cols.next().expect("lemma col");
                let commonality = cols.next().expect("commonality col");
                let part_of_speech = cols.next().expect("part_of_speech col");
                let dictionary_info = cols.next().expect("dictionary_info col");

                match part_of_speech {
                    "Verb" => {
                        let verb: rubit_api_db::dictionary_info::russian::RussianVerb = serde_json::from_str(dictionary_info).expect("success of deserialization");
                        if let Some(form) = verb.я_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.ты_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.он_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.мы_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.вы_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.они_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.masc_past { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.fem_past { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.neut_past { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.plur_past { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.past_passive { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.past_adverbial { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.past_active { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.present_active { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.present_adverbial { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.singular_imperative { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.plural_imperative { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                    },
                    "Noun" => {
                        let noun: rubit_api_db::dictionary_info::russian::RussianNoun = serde_json::from_str(dictionary_info).expect("success of deserialization");
                        if let Some(form) = noun.nom_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.nom_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.acc_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.acc_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.gen_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.gen_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.ins_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.ins_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.pre_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.pre_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                    },
                    "Adjective" => {
                        let adj: rubit_api_db::dictionary_info::russian::RussianAdjective = serde_json::from_str(dictionary_info).expect("success of deserialization");
                        writer.write(format!("{}|{}\n", adj.nom_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.nom_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.nom_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.nom_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.acc_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.acc_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.acc_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.acc_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.gen_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.gen_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.gen_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.gen_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.ins_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.ins_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.ins_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.ins_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.pre_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.pre_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.pre_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.pre_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.dat_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.dat_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.dat_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.dat_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        if let Some(form) = adj.m_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = adj.f_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = adj.n_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = adj.p_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };

                    },
                    _ => panic!("Unrecognized type encountered in the Type (part of speech) column: {part_of_speech}")
                }
            }
        }
        Language::Ukrainian => {
            for row in rows {
                let mut cols = row.split("|");
                let lemma = cols.next().expect("lemma col");
                let commonality = cols.next().expect("commonality col");
                let part_of_speech = cols.next().expect("part_of_speech col");
                let dictionary_info = cols.next().expect("dictionary_info col");

                match part_of_speech {
                    "Verb" => {
                        let verb: rubit_api_db::dictionary_info::ukrainian::UkrainianVerb = serde_json::from_str(dictionary_info).expect("success of deserialization");
                        if let Some(form) = verb.я_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.ти_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.він_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.ми_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.ви_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.вони_form { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.masc_past { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.fem_past { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.neut_past { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.plur_past { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.past_passive { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.past_adverbial { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.past_active { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.present_active { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.present_adverbial { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.ти_imperative { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = verb.ви_imperative { writer.write(format!("{}|{}\n", form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); };
                    },
                    "Noun" => {
                        let noun: rubit_api_db::dictionary_info::ukrainian::UkrainianNoun = serde_json::from_str(dictionary_info).expect("success of deserialization");
                        if let Some(form) = noun.nom_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.nom_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.acc_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.acc_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.gen_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.gen_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.ins_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.ins_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.loc_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.loc_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                    },
                    "Adjective" => {
                        let adj: rubit_api_db::dictionary_info::ukrainian::UkrainianAdjective = serde_json::from_str(dictionary_info).expect("success of deserialization");
                        writer.write(format!("{}|{}\n", adj.nom_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.nom_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.nom_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.nom_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.acc_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.acc_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.acc_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.acc_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.gen_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.gen_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.gen_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.gen_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.ins_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.ins_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.ins_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.ins_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.loc_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.loc_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.loc_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.loc_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.dat_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.dat_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.dat_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.dat_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        if let Some(form) = adj.m_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = adj.f_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = adj.n_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = adj.p_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };

                    },
                    _ => panic!("Unrecognized type encountered in the Type (part of speech) column: {part_of_speech}")
                }
            }
        }
        Language::Belarusian => {
            for row in rows {
                let mut cols = row.split("|");
                let lemma = cols.next().expect("lemma col");
                let commonality = cols.next().expect("commonality col");
                let part_of_speech = cols.next().expect("part_of_speech col");
                let dictionary_info = cols.next().expect("dictionary_info col");

                match part_of_speech {
                    "Verb" => {
                        let verb: rubit_api_db::dictionary_info::belarusian::BelarusianVerb = serde_json::from_str(dictionary_info).expect("success of deserialization");
                        writer.write(format!("{}|{}\n", verb.я_form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.ты_form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.ён_form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.мы_form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.вы_form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.яны_form.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.masc_past.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.fem_past.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.neut_past.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.plur_past.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.past_passive.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.past_adverbial.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.past_active.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.present_active.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.present_adverbial.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.singular_imperative.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                        writer.write(format!("{}|{}\n", verb.plural_imperative.unaccented(), verb.lemma).as_bytes()).expect("writing success"); 
                    },
                    "Noun" => {
                        let noun: rubit_api_db::dictionary_info::belarusian::BelarusianNoun = serde_json::from_str(dictionary_info).expect("success of deserialization");
                        if let Some(form) = noun.nom_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.nom_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.acc_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.acc_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.gen_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.gen_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.ins_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.ins_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.loc_sing { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = noun.loc_plur { writer.write(format!("{}|{}\n", form.unaccented(), noun.lemma).as_bytes()).expect("writing success"); };
                    },
                    "Adjective" => {
                        let adj: rubit_api_db::dictionary_info::belarusian::BelarusianAdjective = serde_json::from_str(dictionary_info).expect("success of deserialization");
                        writer.write(format!("{}|{}\n", adj.nom_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.nom_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.nom_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.nom_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.acc_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.acc_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.acc_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.acc_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.gen_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.gen_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.gen_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.gen_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.ins_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.ins_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.ins_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.ins_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.loc_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.loc_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.loc_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.loc_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        writer.write(format!("{}|{}\n", adj.dat_masc.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.dat_fem.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.dat_neut.unaccented(), adj.lemma).as_bytes()).expect("writing success");
                        writer.write(format!("{}|{}\n", adj.dat_plur.unaccented(), adj.lemma).as_bytes()).expect("writing success");

                        if let Some(form) = adj.m_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = adj.f_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = adj.n_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };
                        if let Some(form) = adj.p_short { writer.write(format!("{}|{}\n", form.unaccented(), adj.lemma).as_bytes()).expect("writing success"); };

                    },
                    _ => panic!("Unrecognized type encountered in the Type (part of speech) column: {part_of_speech}")
                }
            }
        }
    }

    Ok(())
}
