use bevy::prelude::Transform;
use bevy_ggrs::checksum_hasher;
use std::hash::{Hash, Hasher};

// See https://johanhelsing.studio/posts/extreme-bevy-desync-detection
pub fn checksum_transform(transform: &Transform) -> u64 {
    let mut hasher = checksum_hasher();
    assert!(
        transform.is_finite(),
        "Hashing is not stable for NaN f32 value."
    );

    transform.translation.x.to_bits().hash(&mut hasher);
    transform.translation.y.to_bits().hash(&mut hasher);
    transform.translation.z.to_bits().hash(&mut hasher);

    transform.rotation.x.to_bits().hash(&mut hasher);
    transform.rotation.y.to_bits().hash(&mut hasher);
    transform.rotation.z.to_bits().hash(&mut hasher);
    transform.rotation.w.to_bits().hash(&mut hasher);

    hasher.finish()
}
