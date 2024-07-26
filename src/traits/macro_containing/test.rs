use super::*;

#[test]
fn single_macro() {
    let text = String::from("This is a test {{macro}}.");
    let expected = vec![String::from("{{macro}}")];
    assert_eq!(text.find_macros(), expected);
}

#[test]
fn multiple_macros() {
    let text = String::from("Here are two macros: {{be-adj|іра́нскі}} \n\n and0 \0{{infl of|uk|регіона́льний||m//n|dat|s|;|m//n|loc|s}}==============.");
    let expected = vec![String::from("{{be-adj|іра́нскі}}"), String::from("{{infl of|uk|регіона́льний||m//n|dat|s|;|m//n|loc|s}}")];
    assert_eq!(text.find_macros(), expected);
}

#[test]
fn false_macro() {
    let text = String::from("foo bar {  biz {ru-noun+|bar}}        {  {  foo   }   }");
    let expected: Vec<String> = Vec::new();
    assert_eq!(text.find_macros(), expected);
}

#[test]
fn nested_macros() {
    let text = String::from("This has {{nested {{macro}} inside}}.");
    let expected = vec![String::from("{{nested {{macro}} inside}}")];
    assert_eq!(text.find_macros(), expected);
}

#[test]
fn incomplete_macros() {
    let text = String::from("This has an incomplete macro: {{macro.");
    let expected: Vec<String> = Vec::new();
    assert_eq!(text.find_macros(), expected);
}

#[test]
fn no_macros() {
    let text = String::from("This has no macros.");
    let expected: Vec<String> = Vec::new();
    assert_eq!(text.find_macros(), expected);
}

#[test]
fn empty_string() {
    let text = String::from("");
    let expected: Vec<String> = Vec::new();
    assert_eq!(text.find_macros(), expected);
}

#[test]
fn adjacent_macros() {
    let text = String::from("{{macro1}}{{macro2}}");
    let expected = vec![String::from("{{macro1}}"), String::from("{{macro2}}")];
    assert_eq!(text.find_macros(), expected);
}


#[test]
fn many_macros() {
    let text = String::from(r#"
    // {{ru-noun+|b}} == Stress pattern only indicated, meaning use Title as the word :D
    // {{ru-conj|pf|6c+p|оказа́ть}}
    // {{ru-noun-table|Юпи́тер|a=ia}}
    // {{ru-noun+|футбо́лка|*}}
    // {{ru-noun+|моде́ль//modɛ́lʹ|f|a=ai|adj=моде́льный}}
    // {{uk-noun|футбо́лка&lt;*&gt;}}
    // {{be-noun|пёс&lt;b*.anml&gt;}}
    // {{be-noun|ко́шка&lt;*.anml&gt;}}
    // {{be-verb|пасябрава́ць|pf|impf=сябрава́ць}}
    // {{be-verb|вы́ссаць|pf|impf=высыса́ць}} {{tlb|be|transitive}}
    // {{be-adj|іра́нскі}}
    // {{be-adv|сёлета}}
    // UK ADJ, BE ADJ, RU ADJ) - just pull oneth | section
    // {{uk-adj|хоро́ший|кра́щий|comp2=лі́пший|найкра́щий|sup2=найлі́пший|adv=до́бре}}
    // {{uk-adj|вели́кий|бі́льший|найбі́льший|absn=величина́}}
    // {{uk-adj|ли́тий}}
    // {{inflection of|uk|коме́дія||gen|p}}
    // {{inflection of|ru|ка́лий||nom//acc|p|;|pre|s}}
    // {{inflection of|be|во́ка||nom//acc|p}}
    // {{infl of|be|саба́ка||gen//acc|p}}
    // {{infl of|uk|відмо́витися||p|past|ind|g=pf}}
    // {{infl of|uk|дити́нство||loc|s}}
    // {{infl of|uk|регіона́льний||m//n|dat|s|;|m//n|loc|s}}
    "#);
    let expected = 23;
    assert_eq!(text.find_macros().len(), expected);

}