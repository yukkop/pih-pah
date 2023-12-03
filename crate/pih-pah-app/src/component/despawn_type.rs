
#[derive(Debug)]
pub enum DespawnType {
    Position(Option<f32>, Option<f32>, Option<f32>),
    Toggled(bool),
}

pub trait IntoDespawnTypeVec {
    fn into_despawn_type_vec(self) -> Vec<DespawnType>;
}

impl IntoDespawnTypeVec for DespawnType {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self]
    }
}

impl<A: Into<DespawnType>, B: Into<DespawnType>> IntoDespawnTypeVec for (A, B) {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self.0.into(), self.1.into()]
    }
}

impl<A: Into<DespawnType>, B: Into<DespawnType>, C: Into<DespawnType>> IntoDespawnTypeVec for (A, B, C) {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}

impl<A: Into<DespawnType>, B: Into<DespawnType>, C: Into<DespawnType>, D: Into<DespawnType>> IntoDespawnTypeVec for (A, B, C, D) {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}

impl<A: Into<DespawnType>, B: Into<DespawnType>, C: Into<DespawnType>, D: Into<DespawnType>, E: Into<DespawnType>> IntoDespawnTypeVec for (A, B, C, D, E) {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into()]
    }
}

impl <A: Into<DespawnType>, B: Into<DespawnType>, C: Into<DespawnType>, D: Into<DespawnType>, E: Into<DespawnType>, F: Into<DespawnType>> IntoDespawnTypeVec for (A, B, C, D, E, F) {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into()]
    }
}

impl <A: Into<DespawnType>, B: Into<DespawnType>, C: Into<DespawnType>, D: Into<DespawnType>, E: Into<DespawnType>, F: Into<DespawnType>, G: Into<DespawnType>> IntoDespawnTypeVec for (A, B, C, D, E, F, G) {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into()]
    }
}

impl <A: Into<DespawnType>, B: Into<DespawnType>, C: Into<DespawnType>, D: Into<DespawnType>, E: Into<DespawnType>, F: Into<DespawnType>, G: Into<DespawnType>, H: Into<DespawnType>> IntoDespawnTypeVec for (A, B, C, D, E, F, G, H) {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into()]
    }
}

impl <A: Into<DespawnType>, B: Into<DespawnType>, C: Into<DespawnType>, D: Into<DespawnType>, E: Into<DespawnType>, F: Into<DespawnType>, G: Into<DespawnType>, H: Into<DespawnType>, I: Into<DespawnType>> IntoDespawnTypeVec for (A, B, C, D, E, F, G, H, I) {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into()]
    }
    
}

impl <A: Into<DespawnType>, B: Into<DespawnType>, C: Into<DespawnType>, D: Into<DespawnType>, E: Into<DespawnType>, F: Into<DespawnType>, G: Into<DespawnType>, H: Into<DespawnType>, I: Into<DespawnType>, J: Into<DespawnType>> IntoDespawnTypeVec for (A, B, C, D, E, F, G, H, I, J) {
    fn into_despawn_type_vec(self) -> Vec<DespawnType> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into()]
    }
    
}