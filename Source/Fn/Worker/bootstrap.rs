//! Worker bootstrap code generation
//!
//! Generates the bootstrap code needed to run web workers.

use super::{WorkerConfig, WorkerType};

/// Generates bootstrap code for a web worker
pub struct WorkerBootstrap {
	config:WorkerConfig,
}

impl WorkerBootstrap {
	pub fn new(config:WorkerConfig) -> Self { Self { config } }

	/// Generate bootstrap code for a module worker
	pub fn generate_module_worker(&self, entry_point:&str) -> String {
		let mut code = String::new();

		// Add shebang for ES modules in workers
		code.push_str("// Module worker bootstrap\n");

		// Add polyfills and global setup
		code.push_str(&self.generate_polyfills());

		// Add bootstrap scripts
		for script in &self.config.bootstrap_scripts {
			code.push_str(&format!("import '{}';\n", script));
		}

		// Add the main entry point
		code.push_str(&format!("import '{}';\n", entry_point));

		code
	}

	/// Generate bootstrap code for a classic worker
	pub fn generate_classic_worker(&self, entry_point:&str) -> String {
		let mut code = String::new();

		// Add shebang
		code.push_str("// Classic worker bootstrap\n");

		// Add polyfills for classic workers
		code.push_str(&self.generate_classic_polyfills());

		// Add bootstrap scripts
		for script in &self.config.bootstrap_scripts {
			code.push_str(&format!("importScripts('{}');\n", script));
		}

		// Add the main entry point
		code.push_str(&format!("importScripts('{}');\n", entry_point));

		code
	}

	/// Generate bootstrap for a shared worker
	pub fn generate_shared_worker(&self, entry_point:&str) -> String {
		let mut code = String::new();

		code.push_str("// Shared worker bootstrap\n");

		// Shared workers need port handling
		code.push_str(
			r#"
self.onconnect = function(event) {
    const port = event.ports[0];
    port.onmessage = function(event) {
        // Handle messages from the main thread
        self.dispatchEvent(new MessageEvent('message', event));
    };
    port.start();
};

"#,
		);

		// Add polyfills
		code.push_str(&self.generate_classic_polyfills());

		// Add the main entry point
		code.push_str(&format!("importScripts('{}');\n", entry_point));

		code
	}

	/// Generate polyfills for module workers
	fn generate_polyfills(&self) -> String {
		r#"
// Polyfills for worker environment
(function() {
    // Ensure globalThis is available
    if (typeof globalThis === 'undefined') {
        self.globalThis = self;
    }
    
    // Ensure MessageChannel is available
    if (typeof MessageChannel === 'undefined') {
        self.MessageChannel = class MessageChannel {
            constructor() {
                this.port1 = new MessagePort();
                this.port2 = new MessagePort();
            }
        };
    }
    
    // Ensure MessagePort is available
    if (typeof MessagePort === 'undefined') {
        self.MessagePort = class MessagePort {
            constructor() {
                this.onmessage = null;
                this.onmessageerror = null;
            }
            postMessage(data) {}
            start() {}
            close() {}
        };
    }
})();

"#
		.to_string()
	}

	/// Generate polyfills for classic workers
	fn generate_classic_polyfills(&self) -> String {
		r#"
// Classic worker polyfills
(function() {
    // Minimal polyfills for classic workers
    if (typeof globalThis === 'undefined') {
        self.globalThis = self;
    }
})();

"#
		.to_string()
	}

	/// Generate a worker loader script that creates workers from modules
	pub fn generate_worker_loader(&self, worker_name:&str, module_url:&str) -> String {
		format!(
			r#"
(function() {{
    const workerCode = `
        {loader_code}
    `;
    
    const blob = new Blob([workerCode], {{ type: 'application/javascript' }});
    const url = URL.createObjectURL(blob);
    
    self["{worker_name}"] = new Worker(url, {{ type: 'module' }});
    
    // Clean up blob URL after worker is created
    URL.revokeObjectURL(url);
}})();
"#,
			loader_code = self
				.generate_module_worker(module_url)
				.replace("`", "\\`")
				.replace("${", "\\${")
		)
	}
}

/// Generate inline worker code for small workers
pub fn generate_inline_worker(code:&str, worker_type:WorkerType) -> String {
	match worker_type {
		WorkerType::Module => {
			format!(
				"new Worker(URL.createObjectURL(new Blob([`{}`], {{ type: 'application/javascript' }})), {{ type: \
				 'module' }})",
				code.replace("`", "\\`").replace("${", "\\${")
			)
		},
		WorkerType::Classic => {
			format!(
				"new Worker(URL.createObjectURL(new Blob([`{}`], {{ type: 'application/javascript' }})))",
				code.replace("`", "\\`").replace("${", "\\${")
			)
		},
	}
}

/// Generate a TypeScript declaration for worker imports
pub fn generate_worker_declaration(worker_name:&str) -> String {
	format!(
		r#"declare const {worker_name}: Worker;
export {{ {worker_name} }};
"#
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_module_worker_bootstrap() {
		let config = WorkerConfig::new();
		let bootstrap = WorkerBootstrap::new(config);

		let code = bootstrap.generate_module_worker("./entry.js");
		assert!(code.contains("Module worker bootstrap"));
		assert!(code.contains("./entry.js"));
	}

	#[test]
	fn test_classic_worker_bootstrap() {
		let config = WorkerConfig::new();
		let bootstrap = WorkerBootstrap::new(config);

		let code = bootstrap.generate_classic_worker("./entry.js");
		assert!(code.contains("Classic worker bootstrap"));
		assert!(code.contains("./entry.js"));
	}

	#[test]
	fn test_shared_worker_bootstrap() {
		let config = WorkerConfig::new();
		let bootstrap = WorkerBootstrap::new(config);

		let code = bootstrap.generate_shared_worker("./entry.js");
		assert!(code.contains("Shared worker bootstrap"));
		assert!(code.contains("onconnect"));
	}
}
