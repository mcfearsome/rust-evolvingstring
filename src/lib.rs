use sha2::{Sha256, Digest};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EvolvingString {
    pub initial_string: String,
    pub secret: String,
    pub interval_seconds: u64,
    #[serde(with = "system_time_serde")]
    pub start_time: SystemTime,
}

mod system_time_serde {
    use std::time::{UNIX_EPOCH, Duration, SystemTime};
    use serde::{Deserialize, Serializer, Deserializer};

    // Serde serialization and deserialization for SystemTime.
    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let duration_since_epoch = time.duration_since(UNIX_EPOCH).map_err(serde::ser::Error::custom)?;
        let secs = duration_since_epoch.as_secs();
        serializer.serialize_u64(secs)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where D: Deserializer<'de> {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + Duration::from_secs(secs))
    }
}

impl EvolvingString {
    pub fn new(initial_string: String, secret: String, interval_seconds: u64) -> Self {
        Self {
            initial_string,
            secret,
            interval_seconds,
            start_time: SystemTime::now(),
        }
    }
    
    pub fn current(&self) -> String {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.start_time).expect("Time went backwards");
        let interval_count = elapsed.as_secs() / self.interval_seconds;
        self.evolve(interval_count)
    }
    
    pub fn predict(&self, n_seconds: u64) -> String {
        let interval_count = n_seconds / self.interval_seconds;
        self.evolve(interval_count)
    }

    fn evolve(&self, interval_count: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.initial_string);
        hasher.update(&self.secret);
        hasher.update(interval_count.to_be_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn to_base64(&self) -> String {
        let serialized = serde_json::to_string(&self).expect("Failed to serialize EvolvingString");
        base64::encode(serialized)
    }

    pub fn from_base64(b64_encoded: &str) -> Result<Self, String> {
        match base64::decode(b64_encoded) {
            Ok(bytes) => {
                match serde_json::from_slice(&bytes) {
                    Ok(es) => Ok(es),
                    Err(e) => Err(format!("Deserialization error: {}", e)),
                }
            }
            Err(_) => Err("Base64 decoding error".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn new_creates_correct_instance() {
        let es = EvolvingString::new("test".to_string(), "secret".to_string(), 60);
        assert_eq!(es.initial_string, "test");
        assert_eq!(es.secret, "secret");
        assert_eq!(es.interval_seconds, 60);
    }

    #[test]
    fn current_matches_predicted_string() {
        let es = EvolvingString::new("test".to_string(), "secret".to_string(), 1);
        let predicted = es.predict(2);
        sleep(Duration::from_secs(2));
        assert_eq!(predicted, es.current());
    }

    #[test]
    fn evolve_produces_consistent_output() {
        let es = EvolvingString::new("test".to_string(), "secret".to_string(), 60);
        let output1 = es.evolve(0);
        let output2 = es.evolve(0);
        assert_eq!(output1, output2);
    }

    #[test]
    fn to_base64_encodes_correctly() {
        let es = EvolvingString::new("test".to_string(), "secret".to_string(), 60);
        let b64 = es.to_base64();
        assert!(b64.len() > 0);
    }

    #[test]
    fn from_base64_decodes_correctly() {
        let es_original = EvolvingString::new("test".to_string(), "secret".to_string(), 60);
        let b64 = es_original.to_base64();
        let es_decoded = EvolvingString::from_base64(&b64).unwrap();
        assert_eq!(es_original.initial_string, es_decoded.initial_string);
        assert_eq!(es_original.secret, es_decoded.secret);
        assert_eq!(es_original.interval_seconds, es_decoded.interval_seconds);
    }

    #[test]
    fn serialization_deserialization_is_lossless() {
        let es_original = EvolvingString::new("test".to_string(), "secret".to_string(), 60);
        let b64 = es_original.to_base64();
        let es_decoded = EvolvingString::from_base64(&b64).unwrap();
        let b64_second = es_decoded.to_base64();
        assert_eq!(b64, b64_second);
    }

    #[test]
    fn base64_serialization_roundtrip() {
        let es = EvolvingString::new("test".to_string(), "secret".to_string(), 60);
        let b64 = es.to_base64();
        let decoded_es = EvolvingString::from_base64(&b64).unwrap();
        assert_eq!(es.initial_string, decoded_es.initial_string);
        assert_eq!(es.secret, decoded_es.secret);
        assert_eq!(es.interval_seconds, decoded_es.interval_seconds);
        // Additional validations can be done here
    }
}
