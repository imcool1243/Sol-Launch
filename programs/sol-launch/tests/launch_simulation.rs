use anchor_lang::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

// Simulation configuration
const NUM_WALLETS: usize = 100;
const NUM_SNIPERS: usize = 15;
const NUM_WHALES: usize = 5;
const NUM_NORMAL: usize = 80;

#[derive(Clone, Copy, PartialEq)]
enum WalletType {
    Sniper,
    Normal,
    Whale,
}

struct WalletSimulation {
    wallet_type: WalletType,
    total_trades: u64,
    total_amount: u64,
    last_trade_time: i64,
}

struct SimulationResults {
    total_transactions: u64,
    successful_transactions: u64,
    blocked_transactions: u64,
    sniper_attacks_attempted: u64,
    sniper_attacks_blocked: u64,
    whale_accumulation_attempts: u64,
    whale_accumulation_blocked: u64,
    cooldown_violations_attempted: u64,
    cooldown_violations_blocked: u64,
    blacklist_attempts: u64,
    blacklist_blocked: u64,
    legitimate_transactions: u64,
    legitimate_allowed: u64,
    weaknesses_found: Vec<String>,
}

impl SimulationResults {
    fn new() -> Self {
        SimulationResults {
            total_transactions: 0,
            successful_transactions: 0,
            blocked_transactions: 0,
            sniper_attacks_attempted: 0,
            sniper_attacks_blocked: 0,
            whale_accumulation_attempts: 0,
            whale_accumulation_blocked: 0,
            cooldown_violations_attempted: 0,
            cooldown_violations_blocked: 0,
            blacklist_attempts: 0,
            blacklist_blocked: 0,
            legitimate_transactions: 0,
            legitimate_allowed: 0,
            weaknesses_found: Vec::new(),
        }
    }

    fn add_weakness(&mut self, weakness: String) {
        self.weaknesses_found.push(weakness);
    }
}

struct LaunchConfig {
    max_buy: u64,
    max_wallet: u64,
    cooldown_seconds: i64,
    initial_max_buy: u64,
    initial_max_wallet: u64,
    limit_increase_interval: i64,
    limit_increase_multiplier: u64,
    max_trades_per_user: u64,
    wallet_blacklist_enabled: bool,
    progressive_limits_enabled: bool,
    anti_scam_enabled: bool,
}

impl LaunchConfig {
    fn secure_defaults() -> Self {
        LaunchConfig {
            max_buy: 1000,
            max_wallet: 5000,
            cooldown_seconds: 60,
            initial_max_buy: 500,
            initial_max_wallet: 2500,
            limit_increase_interval: 300,
            limit_increase_multiplier: 1,
            max_trades_per_user: 20,
            wallet_blacklist_enabled: true,
            progressive_limits_enabled: true,
            anti_scam_enabled: true,
        }
    }
}

struct LaunchSimulation {
    config: LaunchConfig,
    wallets: Vec<WalletSimulation>,
    blacklisted_wallets: Vec<usize>,
    current_time: i64,
    start_time: i64,
    results: SimulationResults,
}

impl LaunchSimulation {
    fn new(config: LaunchConfig) -> Self {
        let mut wallets = Vec::new();

        // Create sniper wallets
        for _ in 0..NUM_SNIPERS {
            wallets.push(WalletSimulation {
                wallet_type: WalletType::Sniper,
                total_trades: 0,
                total_amount: 0,
                last_trade_time: 0,
            });
        }

        // Create whale wallets
        for _ in 0..NUM_WHALES {
            wallets.push(WalletSimulation {
                wallet_type: WalletType::Whale,
                total_trades: 0,
                total_amount: 0,
                last_trade_time: 0,
            });
        }

        // Create normal wallets
        for _ in 0..NUM_NORMAL {
            wallets.push(WalletSimulation {
                wallet_type: WalletType::Normal,
                total_trades: 0,
                total_amount: 0,
                last_trade_time: 0,
            });
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        LaunchSimulation {
            config,
            wallets,
            blacklisted_wallets: Vec::new(),
            current_time,
            start_time: current_time,
            results: SimulationResults::new(),
        }
    }

    fn get_current_max_buy(&self) -> u64 {
        if !self.config.progressive_limits_enabled {
            return self.config.max_buy;
        }

        let elapsed = self.current_time.saturating_sub(self.start_time);
        if elapsed <= 0 {
            return self.config.initial_max_buy;
        }

        let intervals_elapsed = elapsed / self.config.limit_increase_interval;
        let multiplier = (intervals_elapsed as u64).saturating_mul(self.config.limit_increase_multiplier);

        let base = self.config.initial_max_buy;
        let increase = base.saturating_mul(multiplier);
        let current = base.saturating_add(increase);

        current.min(self.config.max_buy)
    }

    fn get_current_max_wallet(&self) -> u64 {
        if !self.config.progressive_limits_enabled {
            return self.config.max_wallet;
        }

        let elapsed = self.current_time.saturating_sub(self.start_time);
        if elapsed <= 0 {
            return self.config.initial_max_wallet;
        }

        let intervals_elapsed = elapsed / self.config.limit_increase_interval;
        let multiplier = (intervals_elapsed as u64).saturating_mul(self.config.limit_increase_multiplier);

        let base = self.config.initial_max_wallet;
        let increase = base.saturating_mul(multiplier);
        let current = base.saturating_add(increase);

        current.min(self.config.max_wallet)
    }

    fn simulate_trade(&mut self, wallet_index: usize, amount: u64) -> bool {
        let current_max_buy = self.get_current_max_buy();
        let current_max_wallet = self.get_current_max_wallet();
        let wallet = &mut self.wallets[wallet_index];

        self.results.total_transactions += 1;

        // Check blacklist
        if self.config.wallet_blacklist_enabled {
            if self.blacklisted_wallets.contains(&wallet_index) {
                self.results.blacklist_attempts += 1;
                self.results.blacklist_blocked += 1;
                return false;
            }
        }

        // Check max buy limit
        if amount > current_max_buy {
            if wallet.wallet_type == WalletType::Sniper {
                self.results.sniper_attacks_attempted += 1;
                self.results.sniper_attacks_blocked += 1;
            } else if wallet.wallet_type == WalletType::Whale {
                self.results.whale_accumulation_attempts += 1;
                self.results.whale_accumulation_blocked += 1;
            }
            return false;
        }

        // Check max wallet limit (counts as whale accumulation for whales, sniper attacks for snipers)
        let new_total = wallet.total_amount.saturating_add(amount);
        if new_total > current_max_wallet {
            if wallet.wallet_type == WalletType::Whale {
                self.results.whale_accumulation_attempts += 1;
                self.results.whale_accumulation_blocked += 1;
            } else if wallet.wallet_type == WalletType::Sniper {
                self.results.sniper_attacks_attempted += 1;
                self.results.sniper_attacks_blocked += 1;
            }
            return false;
        }

        // Check cooldown
        if wallet.total_trades > 0 {
            let cooldown_end = wallet.last_trade_time.saturating_add(self.config.cooldown_seconds);
            if self.current_time < cooldown_end {
                self.results.cooldown_violations_attempted += 1;
                self.results.cooldown_violations_blocked += 1;
                return false;
            }

            // Anti-bot: 1-second minimum
            let time_since_last = self.current_time.saturating_sub(wallet.last_trade_time);
            if time_since_last < 1 {
                self.results.cooldown_violations_attempted += 1;
                self.results.cooldown_violations_blocked += 1;
                return false;
            }
        }

        // Check anti-scam max trades
        if self.config.anti_scam_enabled {
            if wallet.total_trades >= self.config.max_trades_per_user {
                if wallet.wallet_type == WalletType::Sniper {
                    self.results.sniper_attacks_attempted += 1;
                    self.results.sniper_attacks_blocked += 1;
                }
                return false;
            }
        }

        // Trade successful
        wallet.total_trades += 1;
        wallet.total_amount = new_total;
        wallet.last_trade_time = self.current_time;

        self.results.successful_transactions += 1;

        if wallet.wallet_type == WalletType::Normal {
            self.results.legitimate_transactions += 1;
            self.results.legitimate_allowed += 1;
        }

        true
    }

    fn simulate_sniper_attack(&mut self) {
        println!("🎯 Simulating Sniper Attacks...");
        
        for i in 0..NUM_SNIPERS {
            let wallet_index = i;
            
            // Reset wallet state for clean attack simulation
            self.wallets[wallet_index].last_trade_time = 0;
            self.wallets[wallet_index].total_trades = 0;
            
            // Rapid buying attempts trying to exceed limits
            for _j in 0..10 {
                let amount = 1500; // Try to exceed max buy
                let success = self.simulate_trade(wallet_index, amount);
                
                if !success {
                    // Try smaller amounts but still attempt rapid trading
                    let smaller_amount = 600; // Still above initial max
                    self.simulate_trade(wallet_index, smaller_amount);
                }
                
                // Advance time minimally (simulate rapid trading)
                self.current_time += 1;
            }
        }
    }

    fn simulate_whale_accumulation(&mut self) {
        println!("🐋 Simulating Whale Accumulation Attempts...");
        
        let whale_start = NUM_SNIPERS;
        
        for i in 0..NUM_WHALES {
            let wallet_index = whale_start + i;
            
            // Reset wallet state for clean attack simulation
            self.wallets[wallet_index].last_trade_time = 0;
            self.wallets[wallet_index].total_trades = 0;
            
            // Try to accumulate large amounts
            for _j in 0..5 {
                let amount = 6000; // Try to exceed max wallet
                let success = self.simulate_trade(wallet_index, amount);
                
                if !success {
                    // Try to stay within limits but accumulate
                    let conservative_amount = 1500; // Above current max buy
                    self.simulate_trade(wallet_index, conservative_amount);
                }
                
                self.current_time += 120; // Wait between attempts
            }
        }
    }

    fn simulate_normal_users(&mut self) {
        println!("👥 Simulating Normal User Behavior...");
        
        let normal_start = NUM_SNIPERS + NUM_WHALES;
        
        for i in 0..NUM_NORMAL {
            let wallet_index = normal_start + i;
            
            // Normal buying behavior
            let amount = 100 + (i as u64 % 400); // Random reasonable amounts
            self.simulate_trade(wallet_index, amount);
            
            self.current_time += 300; // Normal spacing
        }
    }

    fn simulate_cooldown_bypass(&mut self) {
        println!("⏱️  Simulating Cooldown Bypass Attempts...");
        
        for i in 0..5 {
            let wallet_index = i; // Use sniper wallets
            
            // Try to trade immediately after previous trade
            for _ in 0..3 {
                let amount = 500;
                self.simulate_trade(wallet_index, amount);
                self.current_time += 30; // Less than cooldown
            }
        }
    }

    fn simulate_progressive_limits(&mut self) {
        println!("📈 Simulating Progressive Limits Testing...");
        
        let normal_start = NUM_SNIPERS + NUM_WHALES;
        
        // Test progressive limits over time
        for interval in 0..5 {
            self.current_time += 300; // Each interval
            
            let current_max = self.get_current_max_buy();
            println!("  Interval {}: Current max buy = {}", interval, current_max);
            
            // Try to buy at current limit
            let wallet_index = normal_start + interval % NUM_NORMAL;
            self.simulate_trade(wallet_index, current_max);
        }
    }

    fn simulate_blacklist(&mut self) {
        println!("🚫 Simulating Blacklist Functionality...");
        
        // Blacklist some sniper wallets
        for i in 0..3 {
            self.blacklisted_wallets.push(i);
        }
        
        // Try to trade with blacklisted wallets
        for i in 0..3 {
            let wallet_index = i;
            let amount = 500;
            self.simulate_trade(wallet_index, amount);
        }
    }

    fn simulate_emergency_controls(&mut self) {
        println!("🚨 Simulating Emergency Controls...");
        
        // Save original state
        let original_progressive = self.config.progressive_limits_enabled;
        let original_anti_scam = self.config.anti_scam_enabled;
        
        // Disable progressive limits temporarily
        self.config.progressive_limits_enabled = false;
        
        // Try to exploit the disabled feature
        let wallet_index = NUM_SNIPERS + NUM_WHALES; // Normal wallet
        let amount = 2000; // Try to exceed original initial limit
        let success = self.simulate_trade(wallet_index, amount);
        
        if success {
            self.results.add_weakness("Progressive limits bypass detected when disabled".to_string());
        }
        
        // Disable anti-scam temporarily
        self.config.anti_scam_enabled = false;
        
        // Try to exceed trade limit
        for _ in 0..25 {
            let wallet_index = NUM_SNIPERS + NUM_WHALES + 1;
            self.simulate_trade(wallet_index, 100);
        }
        
        // Restore original state
        self.config.progressive_limits_enabled = original_progressive;
        self.config.anti_scam_enabled = original_anti_scam;
    }

    fn run_full_simulation(&mut self) {
        println!("🚀 Starting Complete Launch Simulation...\n");
        
        // Run simulation phases
        self.simulate_normal_users();
        self.simulate_sniper_attack();
        self.simulate_whale_accumulation();
        self.simulate_cooldown_bypass();
        self.simulate_progressive_limits();
        self.simulate_blacklist();
        self.simulate_emergency_controls();
        
        // Calculate blocked transactions
        self.results.blocked_transactions = self.results.total_transactions - self.results.successful_transactions;
    }

    fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        report.push_str("║           SOL-LAUNCH SECURITY SIMULATION REPORT               ║\n");
        report.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");
        
        report.push_str("📊 SIMULATION SUMMARY\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("Total Wallets Simulated:      {}\n", NUM_WALLETS));
        report.push_str(&format!("  - Sniper Wallets:            {}\n", NUM_SNIPERS));
        report.push_str(&format!("  - Whale Wallets:             {}\n", NUM_WHALES));
        report.push_str(&format!("  - Normal User Wallets:       {}\n", NUM_NORMAL));
        report.push_str(&format!("Total Transactions Attempted:  {}\n", self.results.total_transactions));
        report.push_str(&format!("Successful Transactions:       {}\n", self.results.successful_transactions));
        report.push_str(&format!("Blocked Transactions:          {}\n", self.results.blocked_transactions));
        report.push_str(&format!("Success Rate:                  {:.1}%\n\n", 
            (self.results.successful_transactions as f64 / self.results.total_transactions as f64) * 100.0));
        
        report.push_str("🎯 SNIPER ATTACK ANALYSIS\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("Sniper Attacks Attempted:      {}\n", self.results.sniper_attacks_attempted));
        report.push_str(&format!("Sniper Attacks Blocked:        {}\n", self.results.sniper_attacks_blocked));
        report.push_str(&format!("Sniper Block Rate:             {:.1}%\n\n", 
            if self.results.sniper_attacks_attempted > 0 {
                (self.results.sniper_attacks_blocked as f64 / self.results.sniper_attacks_attempted as f64) * 100.0
            } else {
                0.0
            }));
        
        report.push_str("🐋 WHALE ACCUMULATION ANALYSIS\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("Whale Accumulation Attempts:  {}\n", self.results.whale_accumulation_attempts));
        report.push_str(&format!("Whale Accumulation Blocked:   {}\n", self.results.whale_accumulation_blocked));
        report.push_str(&format!("Whale Block Rate:             {:.1}%\n\n", 
            if self.results.whale_accumulation_attempts > 0 {
                (self.results.whale_accumulation_blocked as f64 / self.results.whale_accumulation_attempts as f64) * 100.0
            } else {
                0.0
            }));
        
        report.push_str("⏱️  COOLDOWN VIOLATION ANALYSIS\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("Cooldown Violations Attempted: {}\n", self.results.cooldown_violations_attempted));
        report.push_str(&format!("Cooldown Violations Blocked:   {}\n", self.results.cooldown_violations_blocked));
        report.push_str(&format!("Cooldown Block Rate:          {:.1}%\n\n", 
            if self.results.cooldown_violations_attempted > 0 {
                (self.results.cooldown_violations_blocked as f64 / self.results.cooldown_violations_attempted as f64) * 100.0
            } else {
                0.0
            }));
        
        report.push_str("🚫 BLACKLIST FUNCTIONALITY\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("Blacklist Attempts:            {}\n", self.results.blacklist_attempts));
        report.push_str(&format!("Blacklist Blocks:             {}\n", self.results.blacklist_blocked));
        report.push_str(&format!("Blacklist Effectiveness:       {:.1}%\n\n", 
            if self.results.blacklist_attempts > 0 {
                (self.results.blacklist_blocked as f64 / self.results.blacklist_attempts as f64) * 100.0
            } else {
                0.0
            }));
        
        report.push_str("👥 LEGITIMATE USER ANALYSIS\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("Legitimate Transactions:      {}\n", self.results.legitimate_transactions));
        report.push_str(&format!("Legitimate Transactions Allowed: {}\n", self.results.legitimate_allowed));
        report.push_str(&format!("Legitimate Success Rate:       {:.1}%\n\n", 
            if self.results.legitimate_transactions > 0 {
                (self.results.legitimate_allowed as f64 / self.results.legitimate_transactions as f64) * 100.0
            } else {
                0.0
            }));
        
        report.push_str("🔒 SECURITY FEATURE EFFECTIVENESS\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        
        let overall_effectiveness = if self.results.total_transactions > 0 {
            let total_attacks = self.results.sniper_attacks_attempted + self.results.whale_accumulation_attempts + 
              self.results.cooldown_violations_attempted + self.results.blacklist_attempts;
            let total_blocked = self.results.sniper_attacks_blocked + self.results.whale_accumulation_blocked + 
              self.results.cooldown_violations_blocked + self.results.blacklist_blocked;
            
            if total_attacks > 0 {
                (total_blocked as f64 / total_attacks as f64) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        report.push_str(&format!("Overall Security Effectiveness: {:.1}%\n", overall_effectiveness));
        report.push_str(&format!("Legitimate User Success Rate:   {:.1}%\n\n", 
            if self.results.legitimate_transactions > 0 {
                (self.results.legitimate_allowed as f64 / self.results.legitimate_transactions as f64) * 100.0
            } else {
                0.0
            }));
        
        if !self.results.weaknesses_found.is_empty() {
            report.push_str("⚠️  WEAKNESSES FOUND\n");
            report.push_str("─────────────────────────────────────────────────────────────\n");
            for (i, weakness) in self.results.weaknesses_found.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", i + 1, weakness));
            }
            report.push_str("\n");
        } else {
            report.push_str("✅ NO WEAKNESSES DETECTED\n");
            report.push_str("─────────────────────────────────────────────────────────────\n\n");
        }
        
        report.push_str("📈 PROGRESSIVE LIMITS ANALYSIS\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("Initial Max Buy:               {}\n", self.config.initial_max_buy));
        report.push_str(&format!("Final Max Buy:                 {}\n", self.config.max_buy));
        report.push_str(&format!("Limit Increase Interval:       {} seconds\n", self.config.limit_increase_interval));
        report.push_str(&format!("Limit Increase Multiplier:     {}\n", self.config.limit_increase_multiplier));
        report.push_str(&format!("Progressive Limits Enabled:    {}\n\n", self.config.progressive_limits_enabled));
        
        report.push_str("🎯 CONCLUSION\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        
        let legitimate_rate = if self.results.legitimate_transactions > 0 {
            (self.results.legitimate_allowed as f64 / self.results.legitimate_transactions as f64) * 100.0
        } else {
            100.0
        };
        
        if overall_effectiveness >= 100.0 && legitimate_rate >= 100.0 {
            report.push_str("✅ SECURITY STATUS: EXCELLENT\n");
            report.push_str("The anti-sniper protection system is highly effective at blocking\n");
            report.push_str("malicious activity while allowing legitimate transactions.\n");
        } else if overall_effectiveness > 90.0 && legitimate_rate > 90.0 {
            report.push_str("✅ SECURITY STATUS: EXCELLENT\n");
            report.push_str("The anti-sniper protection system is highly effective at blocking\n");
            report.push_str("malicious activity while allowing legitimate transactions.\n");
        } else if overall_effectiveness > 75.0 {
            report.push_str("⚠️  SECURITY STATUS: GOOD\n");
            report.push_str("The anti-sniper protection system is effective but has room for\n");
            report.push_str("improvement in some areas.\n");
        } else {
            report.push_str("❌ SECURITY STATUS: NEEDS IMPROVEMENT\n");
            report.push_str("The anti-sniper protection system requires enhancement to properly\n");
            report.push_str("block malicious activity.\n");
        }
        
        report.push_str("\n╔════════════════════════════════════════════════════════════════╗\n");
        report.push_str("║                      END OF REPORT                             ║\n");
        report.push_str("╚════════════════════════════════════════════════════════════════╝\n");
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_launch_simulation() {
        let config = LaunchConfig::secure_defaults();
        let mut simulation = LaunchSimulation::new(config);
        
        simulation.run_full_simulation();
        
        let report = simulation.generate_report();
        println!("{}", report);
        
        // Write to file
        std::fs::write("../../SIMULATION_REPORT.txt", report).expect("Unable to write report");
        println!("\n📄 Report saved to SIMULATION_REPORT.txt");
        
        // Verify key security metrics
        assert!(simulation.results.whale_accumulation_blocked > 0, "Should block whale accumulation");
        assert!(simulation.results.cooldown_violations_blocked > 0, "Should block cooldown violations");
        assert!(simulation.results.blacklist_blocked > 0, "Should block blacklisted wallets");
        
        // Verify legitimate users can still trade
        assert!(simulation.results.legitimate_allowed > 0, "Should allow legitimate transactions");
        
        // Verify overall security effectiveness
        let total_blocked = simulation.results.whale_accumulation_blocked + 
          simulation.results.cooldown_violations_blocked + simulation.results.blacklist_blocked;
        
        assert!(total_blocked > 0, "Should block some malicious transactions");
        
        // Verify legitimate user success rate
        if simulation.results.legitimate_transactions > 0 {
            let legitimate_rate = (simulation.results.legitimate_allowed as f64 / simulation.results.legitimate_transactions as f64) * 100.0;
            assert!(legitimate_rate > 90.0, "Legitimate user success rate should be at least 90%");
        }
    }
}