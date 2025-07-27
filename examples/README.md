# 🌟 Utopia Examples

This directory contains comprehensive examples demonstrating Utopia's revolutionary multi-language capabilities.

## 📚 **Example Categories**

### **🚀 Basic Examples**
- **`basic_example.uto`** - Variables, functions, and control flow
- **`simple_test.uto`** - Basic syntax and operations
- **`unified_syntax_simple.uto`** - Simple cross-language syntax

### **🎯 Advanced Features** 
- **`multi_lang_showcase.uto`** - Multi-language integration examples
- **`advanced_multi_language_demo.uto`** - Enterprise-grade workflows
- **`production_multi_lang_demo.uto`** - Production-ready examples

### **🏢 Enterprise Examples**
- **`web_api.uto`** - Web API development with multiple languages
- **`data_processing.uto`** - Data processing and analysis workflows

### **🔬 Advanced Demos** (Root Directory)
- **`enterprise_demo.uto`** - Real-world business workflows
- **`final_advanced_demo.uto`** - Complete multi-language system
- **`advanced_generics_demo.uto`** - Advanced type system features
- **`utopia_test_framework.uto`** - Comprehensive testing examples

## 🚀 **Running Examples**

### **Basic Compilation**
```bash
# Compile to Python
utopia compile basic_example.uto --target python --output example.py

# Compile to JavaScript  
utopia compile multi_lang_showcase.uto --target javascript --output example.js

# Compile to any of 50+ languages
utopia compile web_api.uto --target rust --output api.rs
```

### **Cross-Language Examples**
```bash
# Enterprise workflow
utopia compile ../enterprise_demo.uto --target python
utopia compile ../enterprise_demo.uto --target javascript  
utopia compile ../enterprise_demo.uto --target java

# Advanced features
utopia compile ../final_advanced_demo.uto --target typescript
```

## 🎯 **Example Highlights**

### **Cross-Language Function Calls**
```utopia
// Python for data processing
@lang python {
def analyze_data(numbers):
    return {"mean": sum(numbers) / len(numbers)}
}

// JavaScript for visualization  
@lang javascript {
function createChart(data) {
    return `Chart: Mean = ${data.mean}`;
}
}

// Rust for performance
@lang rust {
fn fast_computation(n: u64) -> u64 {
    (0..n).sum()
}
}

// Seamless integration
let data = [1, 2, 3, 4, 5]
let analysis = python::analyze_data(data)
let chart = javascript::createChart(analysis)
let result = rust::fast_computation(1000)
```

### **Multi-Language Web API**
```utopia
// Backend in Rust for performance
@lang rust {
fn handle_request(data: &str) -> String {
    format!("Processed: {}", data)
}
}

// Frontend in TypeScript
@lang typescript {
interface ApiResponse {
    success: boolean;
    data: string;
}

function callApi(input: string): ApiResponse {
    return {
        success: true,
        data: `Frontend processing: ${input}`
    };
}
}

// Coordinate with main logic
let request_data = "user_input"
let backend_response = rust::handle_request(request_data)
let frontend_result = typescript::callApi(backend_response)
```

## 📊 **Performance Examples**

### **Language-Optimized Processing**
- **Python**: Data science and AI workflows
- **Rust**: High-performance computations
- **JavaScript**: Web interfaces and real-time updates
- **Java**: Enterprise business logic
- **C**: System-level optimizations

### **Benchmarking**
```bash
# Run performance comparisons
utopia compile data_processing.uto --target python
utopia compile data_processing.uto --target rust
utopia compile data_processing.uto --target c

# Compare execution times across languages
```

## 🛠️ **Development Workflows**

### **Full-Stack Development**
1. **Backend**: Rust for APIs, Python for AI
2. **Frontend**: TypeScript for UI, JavaScript for interactions
3. **Database**: SQL for queries, Python for analysis
4. **Integration**: Seamless cross-language communication

### **Data Science Pipeline**
1. **Collection**: Python for data gathering
2. **Processing**: Rust for performance-critical operations
3. **Analysis**: R for statistical analysis
4. **Visualization**: JavaScript for interactive charts

### **Enterprise Systems**
1. **Business Logic**: Java for reliability
2. **Performance Critical**: C++ for optimization
3. **Integration**: Python for system coordination
4. **APIs**: Go for scalable services

## 🎓 **Learning Path**

### **Beginner**
1. Start with `basic_example.uto`
2. Explore `unified_syntax_simple.uto`
3. Try single-language compilation

### **Intermediate**
1. Study `multi_lang_showcase.uto`
2. Experiment with cross-language calls
3. Build your own multi-language project

### **Advanced**
1. Analyze `advanced_multi_language_demo.uto`
2. Explore enterprise patterns
3. Contribute to the Utopia ecosystem

## 🌟 **Creating Your Own Examples**

### **Template Structure**
```utopia
// Your Multi-Language Project
println("Project Name")

@lang your_language_1 {
    // Language-specific code
}

@lang your_language_2 {
    // Another language
}

// Cross-language integration
let result1 = your_language_1::function_name()
let result2 = your_language_2::function_name()
```

### **Best Practices**
- Use the best language for each task
- Leverage cross-language function calls
- Maintain type safety across boundaries
- Document language-specific optimizations

## 🤝 **Contributing Examples**

We welcome new examples! Please:
1. Follow the existing naming conventions
2. Include comprehensive comments
3. Demonstrate real-world use cases
4. Test compilation across multiple targets

See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed guidelines.

---

**🎯 Explore the infinite possibilities of multi-language programming with Utopia!** 