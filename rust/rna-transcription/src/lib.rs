#[derive(Debug, PartialEq)]
pub struct Rna {
    strain: String,
}

#[derive(Debug, PartialEq)]
pub struct Dna {
    strain: String,
}

impl Rna {
    pub fn new(strain: &str) -> Result<Self, usize> {
        if let Some((i, _)) = strain
            .chars()
            .enumerate()
            .find(|&(_, c)| c != 'G' && c != 'C' && c != 'U' && c != 'A')
        {
            return Err(i);
        }
        Ok(Self {
            strain: strain.to_string(),
        })
    }

    pub fn as_ref(&self) -> &str {
        &self.strain[..]
    }
}

impl Dna {
    pub fn new(strain: &str) -> Result<Self, usize> {
        if let Some((i, _)) = strain
            .chars()
            .enumerate()
            .find(|&(_, c)| c != 'G' && c != 'C' && c != 'T' && c != 'A')
        {
            return Err(i);
        }
        Ok(Self {
            strain: strain.to_string(),
        })
    }

    pub fn as_ref(&self) -> &str {
        &self.strain[..]
    }

    pub fn into_rna(&self) -> Rna {
        let rna_strain = self
            .strain
            .chars()
            .map(|c| match c {
                'C' => 'G',
                'G' => 'C',
                'A' => 'U',
                'T' => 'A',
                _ => unreachable!(), // strain is already validated
            })
            .collect();
        Rna { strain: rna_strain }
    }
}
