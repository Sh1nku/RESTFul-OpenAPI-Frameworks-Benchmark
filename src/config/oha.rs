use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OhaResult {
    pub summary: OhaSummary,
    pub latency_percentiles: OhaLatencyPercentiles,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OhaLatencyPercentiles {
    p_10: f64,
    p_25: f64,
    p_50: f64,
    p_75: f64,
    p_90: f64,
    p_95: f64,
    p_99: f64,
    #[serde(rename = "p99.9")]
    p_99_9: f64,
    #[serde(rename = "p99.99")]
    p_99_99: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OhaSummary {
    pub success_rate: f64,
    pub requests_per_sec: f64,
}

#[cfg(test)]
mod tests {
    use crate::config::oha::OhaResult;

    #[test]
    pub fn verify_oha_deserialize() {
        let result = r#"
{
    "summary": {
        "successRate": 1,
        "total": 10.000950601,
        "slowest": 0.56929248,
        "fastest": 0.18400958,
        "average": 0.20936399922978707,
        "requestsPerSec": 47.99543754890706,
        "totalData": 129730810,
        "sizePerRequest": 276023,
        "sizePerSec": 12971847.894841932
    },
    "responseTimeHistogram": {
        "0.18400958": 1,
        "0.22253787": 419,
        "0.26106616": 40,
        "0.29959445": 0,
        "0.33812274": 0,
        "0.37665103": 0,
        "0.41517932": 0,
        "0.45370761": 0,
        "0.4922359": 0,
        "0.53076419": 0,
        "0.56929248": 10
    },
    "latencyPercentiles": {
        "p10": 0.188865406,
        "p25": 0.193061619,
        "p50": 0.199783871,
        "p75": 0.208389424,
        "p90": 0.223681261,
        "p95": 0.232145465,
        "p99": 0.568546159,
        "p99.9": 0.56929248,
        "p99.99": 0.56929248
    },
    "rps": {
        "mean": 234.28330827798993,
        "stddev": 703.0839631795641,
        "max": 12532.898859508246,
        "min": 1.8357789004634584,
        "percentiles": {
            "p10": 8.016191295567388,
            "p25": 74.99691387698667,
            "p50": 140.033134640312,
            "p75": 236.82548696946003,
            "p90": 377.87392009936013,
            "p95": 590.8663873837936,
            "p99": 1903.5943668847756,
            "p99.9": 12532.898859508246,
            "p99.99": 12532.898859508246
        }
    },
    "details": {
        "DNSDialup": {
            "average": 0.08880289828085107,
            "fastest": 0.073576839,
            "slowest": 0.132091204
        },
        "DNSLookup": {
            "average": 0.00003753089787234041,
            "fastest": 0.000010329,
            "slowest": 0.00080403
        }
    },
    "statusCodeDistribution": {
        "200": 470
    },
    "errorDistribution": {
        "aborted due to deadline": 10
    }
}
"#;
        let _oha_result: OhaResult = serde_json::from_str(result).unwrap();
    }
}
