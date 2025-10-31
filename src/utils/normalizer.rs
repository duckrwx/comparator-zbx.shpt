pub struct Normalizer;

impl Normalizer {
    /// Normaliza um ID removendo prefixos e sufixos comuns
    pub fn normalize_id(id: &str) -> String {
        let id_lower = id.to_lowercase();
        
        // Remover prefixos comuns
        let without_prefix = if id_lower.starts_with("rfeye") {
            &id[5..]
        } else if id_lower.starts_with("miaer-") {
            &id[6..]
        } else if id_lower.starts_with("cwsm") {
            &id[4..]
        } else if id_lower.starts_with("ums") {
            &id[3..]
        } else if id_lower.starts_with("ermx") {
            &id[4..]
        } else {
            id
        };
        
        // Remover zeros à esquerda se for numérico
        if without_prefix.chars().all(|c| c.is_ascii_digit()) {
            without_prefix.trim_start_matches('0').to_string()
        } else {
            id_lower
        }
    }
    
    /// Normaliza texto removendo acentos e caracteres especiais
    pub fn normalize_text(text: &str) -> String {
        text.chars()
            .map(|c| match c {
                'á' | 'à' | 'ã' | 'â' => 'a',
                'é' | 'è' | 'ê' => 'e',
                'í' | 'ì' | 'î' => 'i',
                'ó' | 'ò' | 'õ' | 'ô' => 'o',
                'ú' | 'ù' | 'û' => 'u',
                'ç' => 'c',
                'Á' | 'À' | 'Ã' | 'Â' => 'A',
                'É' | 'È' | 'Ê' => 'E',
                'Í' | 'Ì' | 'Î' => 'I',
                'Ó' | 'Ò' | 'Õ' | 'Ô' => 'O',
                'Ú' | 'Ù' | 'Û' => 'U',
                'Ç' => 'C',
                _ => c,
            })
            .collect()
    }
    
    /// Compara dois IDs de forma flexível
    pub fn ids_match(id1: &str, id2: &str) -> bool {
        let norm1 = Self::normalize_id(id1);
        let norm2 = Self::normalize_id(id2);
        
        // Comparação exata após normalização
        if norm1 == norm2 {
            return true;
        }
        
        // Tentar comparar apenas números se ambos tiverem
        let num1 = norm1.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        let num2 = norm2.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        
        if !num1.is_empty() && num1 == num2 {
            return true;
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_id() {
        assert_eq!(Normalizer::normalize_id("RFeye002300"), "2300");
        assert_eq!(Normalizer::normalize_id("MIAer-SP"), "miaer-sp");
        assert_eq!(Normalizer::normalize_id("CWSM212030"), "212030");
        assert_eq!(Normalizer::normalize_id("00123"), "123");
    }
    
    #[test]
    fn test_normalize_text() {
        assert_eq!(Normalizer::normalize_text("Disponível"), "Disponivel");
        assert_eq!(Normalizer::normalize_text("São Paulo"), "Sao Paulo");
        assert_eq!(Normalizer::normalize_text("AÇÃO"), "ACAO");
    }
    
    #[test]
    fn test_ids_match() {
        assert!(Normalizer::ids_match("RFeye002300", "rfeye002300"));
        assert!(Normalizer::ids_match("RFeye002300", "2300"));
        assert!(Normalizer::ids_match("MIAer-SP", "miaer-sp"));
        assert!(!Normalizer::ids_match("RFeye002300", "RFeye002301"));
    }
}