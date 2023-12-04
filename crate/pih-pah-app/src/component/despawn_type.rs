#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DespawnReason {
    Force,
}

pub trait IntoDespawnTypeVec {
    fn into_despawn_type_vec(self) -> Vec<DespawnReason>;
}

impl IntoDespawnTypeVec for DespawnReason {
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![self]
    }
}

impl<A: Into<DespawnReason>, B: Into<DespawnReason>> IntoDespawnTypeVec for (A, B) {
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![self.0.into(), self.1.into()]
    }
}

impl<A: Into<DespawnReason>, B: Into<DespawnReason>, C: Into<DespawnReason>> IntoDespawnTypeVec
    for (A, B, C)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}

impl<A: Into<DespawnReason>, B: Into<DespawnReason>, C: Into<DespawnReason>, D: Into<DespawnReason>>
    IntoDespawnTypeVec for (A, B, C, D)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
        G: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F, G)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
        G: Into<DespawnReason>,
        H: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F, G, H)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
        G: Into<DespawnReason>,
        H: Into<DespawnReason>,
        I: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F, G, H, I)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
        G: Into<DespawnReason>,
        H: Into<DespawnReason>,
        I: Into<DespawnReason>,
        J: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F, G, H, I, J)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
        ]
    }
}
