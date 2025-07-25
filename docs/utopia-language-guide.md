# 🚀 **Utopia Language Guide**

Your complete guide to mastering the Utopia Multi-Language Compiler. From beginner basics to advanced multi-language integration patterns.

---

## 📚 **Learning Path Overview**

### **🎯 Quick Start (15 minutes)**
Get up and running with your first Utopia program

### **🏗️ Foundation (30 minutes)**
Master core language concepts and syntax

### **🌐 Multi-Language (45 minutes)**
Learn cross-language integration patterns

### **🚀 Advanced (60+ minutes)**
Build complex real-world applications

---

## 🎯 **Quick Start Tutorial**

### **Step 1: Your First Program**

Create a file called `hello.uto`:

```utopia
// Your first Utopia program
println("Hello, Utopia!")
println("Welcome to multi-language programming!")
```

**Run it:**
```bash
utopia run hello.uto
```

**Expected Output:**
```
Hello, Utopia!
Welcome to multi-language programming!
```

🎉 **Congratulations!** You've just written and executed your first Utopia program!

### **Step 2: Variables and Basic Operations**

Create `variables.uto`:

```utopia
// Variables and basic operations
let name = "Alice"
let age = 25
let height = 5.6
let isStudent = true

println("Name:", name)
println("Age:", age)
println("Height:", height, "feet")
println("Is student:", isStudent)

// Basic arithmetic
let currentYear = 2024
let birthYear = currentYear - age
println("Born in:", birthYear)

// String operations
let greeting = "Hello, " + name + "!"
println(greeting)
```

**Key Concepts Learned:**
- ✅ Variable declaration with `let`
- ✅ Different data types (string, number, boolean)
- ✅ Arithmetic operations
- ✅ String concatenation
- ✅ Print statements

### **Step 3: Control Flow**

Create `control.uto`:

```utopia
// Control flow examples
let score = 85

// If statements
if (score >= 90) {
    println("Grade: A - Excellent!")
} else if (score >= 80) {
    println("Grade: B - Good job!")
} else if (score >= 70) {
    println("Grade: C - Keep improving!")
} else {
    println("Grade: F - Study harder!")
}

// Loops
println("\nCountdown:")
for (let i = 5; i > 0; i--) {
    println(i)
}
println("Blast off! 🚀")

// While loop
let count = 0
println("\nCounting to 3:")
while (count < 3) {
    count++
    println("Count:", count)
}
```

**Key Concepts Learned:**
- ✅ Conditional logic with `if/else if/else`
- ✅ For loops with initialization, condition, and increment
- ✅ While loops for conditional iteration
- ✅ Increment/decrement operators

---

## 🏗️ **Foundation Tutorial**

### **Functions and Code Organization**

Create `functions.uto`:

```utopia
// Function definitions
function greet(name) {
    return "Hello, " + name + "!"
}

function calculateArea(length, width) {
    return length * width
}

function isEven(number) {
    return number % 2 == 0
}

// Using functions
let message = greet("Bob")
println(message)

let roomArea = calculateArea(12, 10)
println("Room area:", roomArea, "square feet")

// Function with conditional logic
function describeNumber(num) {
    if (isEven(num)) {
        return num + " is even"
    } else {
        return num + " is odd"
    }
}

for (let i = 1; i <= 5; i++) {
    println(describeNumber(i))
}
```

### **Working with Data Structures**

Create `data-structures.uto`:

```utopia
// Arrays
let fruits = ["apple", "banana", "orange", "grape"]
let numbers = [10, 25, 5, 40, 15]

println("Fruits:", fruits)
println("First fruit:", fruits[0])
println("Last fruit:", fruits[3])

// Array operations
println("\nAll fruits:")
for (let i = 0; i < fruits.length; i++) {
    println((i + 1) + ".", fruits[i])
}

// Finding maximum in array
function findMax(arr) {
    let max = arr[0]
    for (let i = 1; i < arr.length; i++) {
        if (arr[i] > max) {
            max = arr[i]
        }
    }
    return max
}

let maxNumber = findMax(numbers)
println("\nMaximum number:", maxNumber)

// Objects
let person = {
    name: "Alice",
    age: 28,
    city: "San Francisco",
    occupation: "Developer"
}

println("\nPerson info:")
println("Name:", person.name)
println("Age:", person.age)
println("Location:", person.city)
println("Job:", person.occupation)

// Nested objects
let company = {
    name: "Tech Corp",
    founded: 2020,
    headquarters: {
        city: "Seattle",
        state: "WA",
        country: "USA"
    },
    employees: 150
}

println("\nCompany:", company.name)
println("Founded:", company.founded)
println("Location:", company.headquarters.city + ", " + company.headquarters.state)
```

**Key Concepts Learned:**
- ✅ Function definition and calling
- ✅ Return values and parameters
- ✅ Array creation and access
- ✅ Loop through arrays
- ✅ Object creation and property access
- ✅ Nested objects and complex data structures

---

## 🌐 **Multi-Language Integration Tutorial**

### **Your First Multi-Language Program**

Create `multi-lang-basics.uto`:

```utopia
// Python block for data processing
@lang python
import math

def calculate_statistics(numbers):
    total = sum(numbers)
    count = len(numbers)
    average = total / count
    
    # Calculate standard deviation
    variance = sum((x - average) ** 2 for x in numbers) / count
    std_dev = math.sqrt(variance)
    
    return {
        'total': total,
        'count': count,
        'average': average,
        'std_dev': std_dev,
        'min': min(numbers),
        'max': max(numbers)
    }

def format_statistics(stats):
    return f"""
Statistics Summary:
- Count: {stats['count']} numbers
- Total: {stats['total']}
- Average: {stats['average']:.2f}
- Std Dev: {stats['std_dev']:.2f}
- Range: {stats['min']} to {stats['max']}
"""

// JavaScript block for presentation
@lang javascript
function createChart(stats) {
    // Simulating chart creation
    return {
        type: 'bar',
        title: 'Number Statistics',
        data: {
            labels: ['Min', 'Average', 'Max'],
            values: [stats.min, stats.average, stats.max]
        },
        config: {
            responsive: true,
            animated: true
        }
    };
}

function generateReport(stats, chart) {
    const timestamp = new Date().toISOString();
    return {
        title: 'Data Analysis Report',
        generated: timestamp,
        statistics: stats,
        visualization: chart
    };
}

// Main Utopia code
@lang main
let testData = [15, 23, 8, 42, 16, 35, 7, 28, 31, 19]

println("Analyzing data:", testData)

// Use Python for statistical analysis
let stats = py::calculate_statistics(testData)
let summary = py::format_statistics(stats)

// Use JavaScript for visualization
let chart = js::createChart(stats)
let report = js::generateReport(stats, chart)

println(summary)
println("Chart type:", chart.type)
println("Report generated:", report.generated)
```

**Run it:**
```bash
utopia run multi-lang-basics.uto
```

🎉 **Amazing!** You just used Python for mathematical operations and JavaScript for data visualization in a single program!

### **Real-World Example: Weather Analysis**

Create `weather-app.uto`:

```utopia
// Python block for data fetching and analysis
@lang python
import json
from datetime import datetime, timedelta

def simulate_weather_data(city, days=7):
    """Simulate weather data for demo purposes"""
    import random
    
    base_temp = {'New York': 65, 'London': 55, 'Tokyo': 70, 'Sydney': 75}
    temps = []
    
    for i in range(days):
        date = datetime.now() + timedelta(days=i)
        temp = base_temp.get(city, 60) + random.randint(-10, 10)
        humidity = random.randint(30, 90)
        
        temps.append({
            'date': date.strftime('%Y-%m-%d'),
            'temperature': temp,
            'humidity': humidity,
            'condition': random.choice(['Sunny', 'Cloudy', 'Rainy', 'Partly Cloudy'])
        })
    
    return temps

def analyze_weather(weather_data):
    """Analyze weather patterns"""
    temperatures = [day['temperature'] for day in weather_data]
    
    return {
        'avg_temp': sum(temperatures) / len(temperatures),
        'min_temp': min(temperatures),
        'max_temp': max(temperatures),
        'temp_range': max(temperatures) - min(temperatures),
        'total_days': len(weather_data)
    }

def weather_recommendation(analysis):
    """Provide weather-based recommendations"""
    avg_temp = analysis['avg_temp']
    temp_range = analysis['temp_range']
    
    if avg_temp > 75:
        clothing = "Light clothing, shorts, t-shirt"
    elif avg_temp > 60:
        clothing = "Moderate clothing, jeans, light jacket"
    else:
        clothing = "Warm clothing, jacket, sweater"
    
    if temp_range > 20:
        advice = "Expect significant temperature changes - layer your clothing!"
    else:
        advice = "Fairly stable temperatures expected."
    
    return {
        'clothing': clothing,
        'advice': advice
    }

// JavaScript block for user interface
@lang javascript
function formatWeatherDisplay(weatherData, analysis, recommendations) {
    const days = weatherData.map(day => {
        return `${day.date}: ${day.temperature}°F, ${day.condition} (${day.humidity}% humidity)`;
    }).join('\n');
    
    const analysisText = `
Weather Analysis:
Average Temperature: ${analysis.avg_temp.toFixed(1)}°F
Temperature Range: ${analysis.min_temp}°F to ${analysis.max_temp}°F
Variation: ${analysis.temp_range}°F

Recommendations:
Clothing: ${recommendations.clothing}
Advice: ${recommendations.advice}
    `;
    
    return {
        daily: days,
        summary: analysisText
    };
}

function createWeatherAlert(analysis) {
    const avgTemp = analysis.avg_temp;
    
    if (avgTemp > 85) {
        return {
            level: 'warning',
            message: 'Hot weather expected - stay hydrated and avoid prolonged sun exposure'
        };
    } else if (avgTemp < 40) {
        return {
            level: 'warning', 
            message: 'Cold weather expected - dress warmly and watch for ice'
        };
    } else {
        return {
            level: 'info',
            message: 'Pleasant weather conditions expected'
        };
    }
}

// Main application
@lang main
let cities = ["New York", "London", "Tokyo"]

println("🌤️  Weather Analysis Application")
println("=" * 40)

for (let i = 0; i < cities.length; i++) {
    let city = cities[i]
    println("\n📍 " + city.toUpperCase())
    println("-" * 20)
    
    // Fetch weather data using Python
    let weatherData = py::simulate_weather_data(city, 5)
    let analysis = py::analyze_weather(weatherData)
    let recommendations = py::weather_recommendation(analysis)
    
    // Format display using JavaScript
    let display = js::formatWeatherDisplay(weatherData, analysis, recommendations)
    let alert = js::createWeatherAlert(analysis)
    
    // Display results
    println("Daily Forecast:")
    println(display.daily)
    println(display.summary)
    
    println("⚠️  Alert (" + alert.level + "):")
    println(alert.message)
    println("\n" + "=" * 40)
}
```

**Key Concepts Learned:**
- ✅ Multi-language code organization
- ✅ Data flow between languages
- ✅ Real-world application structure
- ✅ Error handling across languages
- ✅ Complex data processing pipelines

---

## 🚀 **Advanced Patterns**

### **Design Pattern: Data Pipeline**

Create `data-pipeline.uto`:

```utopia
// Python: Data ingestion and cleaning
@lang python
import json
import re
from datetime import datetime

class DataCleaner:
    def __init__(self):
        self.processed_count = 0
        self.error_count = 0
    
    def clean_email(self, email):
        """Clean and validate email addresses"""
        if not email or not isinstance(email, str):
            return None
        
        email = email.lower().strip()
        email_pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
        
        if re.match(email_pattern, email):
            return email
        return None
    
    def clean_phone(self, phone):
        """Clean phone numbers"""
        if not phone:
            return None
        
        # Remove all non-digit characters
        digits = re.sub(r'[^\d]', '', str(phone))
        
        # US phone number format
        if len(digits) == 10:
            return f"({digits[:3]}) {digits[3:6]}-{digits[6:]}"
        elif len(digits) == 11 and digits[0] == '1':
            return f"({digits[1:4]}) {digits[4:7]}-{digits[7:]}"
        
        return None
    
    def process_record(self, record):
        """Process a single data record"""
        try:
            cleaned = {
                'name': record.get('name', '').strip().title(),
                'email': self.clean_email(record.get('email')),
                'phone': self.clean_phone(record.get('phone')),
                'age': int(record.get('age', 0)) if str(record.get('age', '')).isdigit() else None,
                'processed_at': datetime.now().isoformat()
            }
            
            # Only return if we have minimum required data
            if cleaned['name'] and (cleaned['email'] or cleaned['phone']):
                self.processed_count += 1
                return cleaned
            else:
                self.error_count += 1
                return None
                
        except Exception as e:
            self.error_count += 1
            return None

def process_data_batch(raw_data):
    """Process a batch of raw data"""
    cleaner = DataCleaner()
    cleaned_records = []
    
    for record in raw_data:
        cleaned = cleaner.process_record(record)
        if cleaned:
            cleaned_records.append(cleaned)
    
    return {
        'records': cleaned_records,
        'stats': {
            'total_input': len(raw_data),
            'successfully_processed': cleaner.processed_count,
            'errors': cleaner.error_count,
            'success_rate': (cleaner.processed_count / len(raw_data)) * 100 if raw_data else 0
        }
    }

// JavaScript: Data validation and transformation
@lang javascript
class DataValidator {
    constructor() {
        this.validationRules = {
            email: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
            phone: /^\(\d{3}\) \d{3}-\d{4}$/,
            age: (age) => age >= 0 && age <= 120
        };
    }
    
    validateRecord(record) {
        const errors = [];
        
        if (!record.name || record.name.length < 2) {
            errors.push('Name must be at least 2 characters');
        }
        
        if (record.email && !this.validationRules.email.test(record.email)) {
            errors.push('Invalid email format');
        }
        
        if (record.phone && !this.validationRules.phone.test(record.phone)) {
            errors.push('Invalid phone format');
        }
        
        if (record.age && !this.validationRules.age(record.age)) {
            errors.push('Age must be between 0 and 120');
        }
        
        return {
            isValid: errors.length === 0,
            errors: errors,
            record: record
        };
    }
    
    validateBatch(records) {
        const results = records.map(record => this.validateRecord(record));
        const validRecords = results.filter(r => r.isValid).map(r => r.record);
        const invalidRecords = results.filter(r => !r.isValid);
        
        return {
            valid: validRecords,
            invalid: invalidRecords,
            summary: {
                total: records.length,
                valid: validRecords.length,
                invalid: invalidRecords.length,
                validationRate: (validRecords.length / records.length) * 100
            }
        };
    }
}

function transformForDatabase(records) {
    return records.map(record => ({
        id: Math.random().toString(36).substr(2, 9),
        full_name: record.name,
        email_address: record.email,
        phone_number: record.phone,
        age_years: record.age,
        created_at: new Date().toISOString(),
        last_updated: new Date().toISOString()
    }));
}

function generateDataReport(processingStats, validationResults) {
    const report = {
        processing: {
            inputRecords: processingStats.total_input,
            cleanedRecords: processingStats.successfully_processed,
            processingErrors: processingStats.errors,
            processingSuccessRate: processingStats.success_rate.toFixed(2) + '%'
        },
        validation: {
            validatedRecords: validationResults.summary.total,
            validRecords: validationResults.summary.valid,
            invalidRecords: validationResults.summary.invalid,
            validationSuccessRate: validationResults.summary.validationRate.toFixed(2) + '%'
        },
        pipeline: {
            totalProcessed: validationResults.summary.valid,
            overallSuccessRate: ((validationResults.summary.valid / processingStats.total_input) * 100).toFixed(2) + '%'
        }
    };
    
    return report;
}

// Main data pipeline
@lang main

// Sample raw data (simulating API input)
let rawData = [
    {name: "john doe", email: "JOHN@EXAMPLE.COM", phone: "5551234567", age: "28"},
    {name: "jane smith", email: "jane.smith@test.co.uk", phone: "555-987-6543", age: "34"},
    {name: "bob", email: "invalid-email", phone: "123", age: "25"},
    {name: "alice johnson", email: "alice@company.org", phone: "(555) 111-2222", age: "29"},
    {name: "", email: "empty@name.com", phone: "5553334444", age: "31"},
    {name: "charlie brown", email: "charlie@example.net", phone: "555.666.7777", age: "150"},
    {name: "diana prince", email: "diana@hero.com", phone: "1-555-888-9999", age: "35"}
]

println("🔄 Data Processing Pipeline Starting...")
println("=" * 50)

// Step 1: Data Cleaning (Python)
println("Step 1: Cleaning raw data...")
let cleaningResult = py::process_data_batch(rawData)
println("✅ Cleaning complete:")
println("  - Input records:", cleaningResult.stats.total_input)
println("  - Successfully cleaned:", cleaningResult.stats.successfully_processed)
println("  - Errors:", cleaningResult.stats.errors)
println("  - Success rate:", cleaningResult.stats.success_rate + "%")

// Step 2: Data Validation (JavaScript)
println("\nStep 2: Validating cleaned data...")
let validator = js::new DataValidator()
let validationResult = js::validator.validateBatch(cleaningResult.records)
println("✅ Validation complete:")
println("  - Valid records:", validationResult.summary.valid)
println("  - Invalid records:", validationResult.summary.invalid)
println("  - Validation rate:", validationResult.summary.validationRate + "%")

// Step 3: Data Transformation (JavaScript)
println("\nStep 3: Transforming for database...")
let dbRecords = js::transformForDatabase(validationResult.valid)
println("✅ Transformation complete:")
println("  - Records ready for database:", dbRecords.length)

// Step 4: Generate Report
println("\nStep 4: Generating pipeline report...")
let report = js::generateDataReport(cleaningResult.stats, validationResult)
println("✅ Pipeline Report:")
println("  Processing Success Rate:", report.processing.processingSuccessRate)
println("  Validation Success Rate:", report.validation.validationSuccessRate)
println("  Overall Success Rate:", report.pipeline.overallSuccessRate)

println("\n🎉 Data pipeline completed successfully!")
println("Final result: " + dbRecords.length + " records ready for storage")

// Display sample transformed record
if (dbRecords.length > 0) {
    println("\nSample transformed record:")
    let sample = dbRecords[0]
    println("  ID:", sample.id)
    println("  Name:", sample.full_name)
    println("  Email:", sample.email_address)
    println("  Phone:", sample.phone_number)
    println("  Age:", sample.age_years)
}
```

### **Advanced Pattern: Microservice Architecture**

Create `microservice-demo.uto`:

```utopia
// Python: Authentication Service
@lang python
import hashlib
import secrets
import json
from datetime import datetime, timedelta

class AuthService:
    def __init__(self):
        self.users = {}
        self.sessions = {}
        self.token_expiry = 3600  # 1 hour
    
    def hash_password(self, password):
        """Hash password with salt"""
        salt = secrets.token_hex(16)
        pwd_hash = hashlib.pbkdf2_hmac('sha256', password.encode(), salt.encode(), 100000)
        return f"{salt}:{pwd_hash.hex()}"
    
    def verify_password(self, stored_password, provided_password):
        """Verify password against stored hash"""
        salt, stored_hash = stored_password.split(':')
        pwd_hash = hashlib.pbkdf2_hmac('sha256', provided_password.encode(), salt.encode(), 100000)
        return stored_hash == pwd_hash.hex()
    
    def register_user(self, username, password, email):
        """Register a new user"""
        if username in self.users:
            return {'success': False, 'error': 'User already exists'}
        
        user_data = {
            'username': username,
            'password_hash': self.hash_password(password),
            'email': email,
            'created_at': datetime.now().isoformat(),
            'last_login': None
        }
        
        self.users[username] = user_data
        return {'success': True, 'message': 'User registered successfully'}
    
    def login(self, username, password):
        """Authenticate user and create session"""
        if username not in self.users:
            return {'success': False, 'error': 'User not found'}
        
        user = self.users[username]
        if not self.verify_password(user['password_hash'], password):
            return {'success': False, 'error': 'Invalid password'}
        
        # Create session token
        token = secrets.token_urlsafe(32)
        session_data = {
            'username': username,
            'created_at': datetime.now().isoformat(),
            'expires_at': (datetime.now() + timedelta(seconds=self.token_expiry)).isoformat()
        }
        
        self.sessions[token] = session_data
        self.users[username]['last_login'] = datetime.now().isoformat()
        
        return {
            'success': True,
            'token': token,
            'expires_in': self.token_expiry,
            'user': {
                'username': username,
                'email': user['email']
            }
        }
    
    def validate_session(self, token):
        """Validate session token"""
        if token not in self.sessions:
            return {'valid': False, 'error': 'Invalid token'}
        
        session = self.sessions[token]
        expires_at = datetime.fromisoformat(session['expires_at'])
        
        if datetime.now() > expires_at:
            del self.sessions[token]
            return {'valid': False, 'error': 'Token expired'}
        
        return {
            'valid': True,
            'username': session['username'],
            'expires_at': session['expires_at']
        }

// JavaScript: API Gateway and Service Orchestration
@lang javascript
class APIGateway {
    constructor() {
        this.routes = new Map();
        this.middleware = [];
        this.rateLimiter = new Map();
    }
    
    addMiddleware(middleware) {
        this.middleware.push(middleware);
    }
    
    registerRoute(path, method, handler) {
        const key = `${method.toUpperCase()}:${path}`;
        this.routes.set(key, handler);
    }
    
    async processRequest(request) {
        // Apply middleware
        for (const middleware of this.middleware) {
            const result = await middleware(request);
            if (!result.continue) {
                return result.response;
            }
            request = result.request;
        }
        
        // Route to handler
        const key = `${request.method}:${request.path}`;
        const handler = this.routes.get(key);
        
        if (!handler) {
            return {
                status: 404,
                body: { error: 'Route not found' }
            };
        }
        
        try {
            return await handler(request);
        } catch (error) {
            return {
                status: 500,
                body: { error: 'Internal server error', message: error.message }
            };
        }
    }
}

class UserService {
    constructor() {
        this.users = new Map();
    }
    
    async getUserProfile(username) {
        if (!this.users.has(username)) {
            // Simulate loading from database
            const profile = {
                username: username,
                profile: {
                    displayName: username.charAt(0).toUpperCase() + username.slice(1),
                    bio: `Software developer passionate about multi-language programming`,
                    joinedDate: new Date().toISOString(),
                    posts: Math.floor(Math.random() * 100),
                    followers: Math.floor(Math.random() * 1000),
                    following: Math.floor(Math.random() * 500)
                },
                preferences: {
                    theme: 'dark',
                    notifications: true,
                    language: 'en'
                }
            };
            this.users.set(username, profile);
        }
        
        return this.users.get(username);
    }
    
    async updateUserProfile(username, updates) {
        const user = await this.getUserProfile(username);
        
        // Merge updates
        user.profile = { ...user.profile, ...updates.profile };
        user.preferences = { ...user.preferences, ...updates.preferences };
        user.profile.updatedAt = new Date().toISOString();
        
        this.users.set(username, user);
        return user;
    }
}

// Middleware functions
function authMiddleware(authService) {
    return async function(request) {
        const publicRoutes = ['/auth/login', '/auth/register', '/health'];
        
        if (publicRoutes.includes(request.path)) {
            return { continue: true, request };
        }
        
        const authHeader = request.headers?.authorization;
        if (!authHeader || !authHeader.startsWith('Bearer ')) {
            return {
                continue: false,
                response: {
                    status: 401,
                    body: { error: 'Authorization required' }
                }
            };
        }
        
        const token = authHeader.substring(7);
        const validation = authService.validate_session(token);
        
        if (!validation.valid) {
            return {
                continue: false,
                response: {
                    status: 401,
                    body: { error: validation.error }
                }
            };
        }
        
        request.user = { username: validation.username };
        return { continue: true, request };
    };
}

function loggingMiddleware() {
    return async function(request) {
        const timestamp = new Date().toISOString();
        console.log(`[${timestamp}] ${request.method} ${request.path} - User: ${request.user?.username || 'anonymous'}`);
        return { continue: true, request };
    };
}

// Main microservice application
@lang main

println("🚀 Microservice Architecture Demo")
println("=" * 40)

// Initialize services
let authService = py::new AuthService()
let userService = js::new UserService()
let gateway = js::new APIGateway()

// Add middleware
gateway.addMiddleware(js::loggingMiddleware())
gateway.addMiddleware(js::authMiddleware(authService))

// Register routes
gateway.registerRoute('/auth/register', 'POST', function(request) {
    let result = authService.register_user(
        request.body.username,
        request.body.password,
        request.body.email
    )
    return {
        status: result.success ? 201 : 400,
        body: result
    }
})

gateway.registerRoute('/auth/login', 'POST', function(request) {
    let result = authService.login(request.body.username, request.body.password)
    return {
        status: result.success ? 200 : 401,
        body: result
    }
})

gateway.registerRoute('/user/profile', 'GET', async function(request) {
    let profile = await userService.getUserProfile(request.user.username)
    return {
        status: 200,
        body: profile
    }
})

gateway.registerRoute('/health', 'GET', function(request) {
    return {
        status: 200,
        body: {
            status: 'healthy',
            timestamp: new Date().toISOString(),
            services: {
                auth: 'online',
                user: 'online',
                gateway: 'online'
            }
        }
    }
})

// Simulate API requests
println("Testing microservice endpoints...\n")

// Test 1: Health check
println("1. Health Check:")
let healthRequest = {
    method: 'GET',
    path: '/health',
    headers: {}
}
let healthResponse = gateway.processRequest(healthRequest)
println("   Status:", healthResponse.status)
println("   Response:", JSON.stringify(healthResponse.body))

// Test 2: User registration
println("\n2. User Registration:")
let registerRequest = {
    method: 'POST',
    path: '/auth/register',
    headers: {},
    body: {
        username: 'alice',
        password: 'secure123',
        email: 'alice@example.com'
    }
}
let registerResponse = gateway.processRequest(registerRequest)
println("   Status:", registerResponse.status)
println("   Response:", JSON.stringify(registerResponse.body))

// Test 3: User login
println("\n3. User Login:")
let loginRequest = {
    method: 'POST',
    path: '/auth/login',
    headers: {},
    body: {
        username: 'alice',
        password: 'secure123'
    }
}
let loginResponse = gateway.processRequest(loginRequest)
println("   Status:", loginResponse.status)
println("   Token received:", loginResponse.body.success ? "✅" : "❌")

// Test 4: Protected endpoint
if (loginResponse.body.success) {
    println("\n4. Accessing Protected Profile:")
    let profileRequest = {
        method: 'GET',
        path: '/user/profile',
        headers: {
            authorization: 'Bearer ' + loginResponse.body.token
        },
        user: { username: 'alice' }
    }
    let profileResponse = gateway.processRequest(profileRequest)
    println("   Status:", profileResponse.status)
    println("   Profile loaded:", profileResponse.status == 200 ? "✅" : "❌")
}

println("\n🎉 Microservice architecture demo completed!")
println("Services working together:")
println("  ✅ Python Authentication Service")
println("  ✅ JavaScript API Gateway")
println("  ✅ JavaScript User Service")
println("  ✅ Cross-language communication")
```

**Key Advanced Concepts Learned:**
- ✅ Multi-service architecture patterns
- ✅ Authentication and session management
- ✅ Middleware and request processing
- ✅ Service orchestration
- ✅ Error handling across services
- ✅ Real-world application design

---

## 🎯 **Best Practices Summary**

### **✅ Code Organization**
```utopia
// 1. Constants and configuration first
const CONFIG = {
    apiUrl: "https://api.example.com",
    timeout: 5000
}

// 2. Helper functions
function validateInput(data) {
    return data != null && data.length > 0
}

// 3. Language blocks by responsibility
@lang python
# Data processing and analysis

@lang javascript  
// UI and presentation logic

@lang main
// Coordination and main application flow
```

### **✅ Error Handling**
```utopia
// Always validate inputs
function divide(a, b) {
    if (b == 0) {
        throw new Error("Division by zero not allowed")
    }
    return a / b
}

// Handle cross-language errors
try {
    let result = py::risky_operation()
    println("Success:", result)
} catch (error) {
    println("Error:", error.message)
}
```

### **✅ Performance Tips**
```utopia
// Use the right language for the job
@lang python
# Heavy computation, data analysis, ML

@lang javascript
# DOM manipulation, async operations, JSON

@lang main
# Simple coordination, basic logic
```

### **✅ Naming Conventions**
```utopia
// Variables: camelCase
let userName = "alice"
let currentUser = getUser()

// Constants: UPPER_SNAKE_CASE  
const MAX_RETRIES = 3
const API_BASE_URL = "https://api.com"

// Functions: camelCase with verbs
function calculateTotal() { }
function validateUser() { }
```

---

## 🏆 **Next Steps**

### **🚀 Build Your Own Project**
1. **Choose a domain** (web app, data analysis, automation)
2. **Identify language strengths** (Python for data, JS for UI)
3. **Design the architecture** (services, data flow)
4. **Start small** (basic functionality first)
5. **Iterate and expand** (add features incrementally)

### **📚 Advanced Learning**
- Read the [**Language Reference**](language-reference.md) for complete syntax
- Explore [**Examples**](examples.md) for more code samples
- Check [**Performance Guide**](performance.md) for optimization
- Join the community for support and collaboration

### **🛠️ Tools and Resources**
- **CLI Reference**: Master all compiler commands
- **Testing Guide**: Learn to write robust tests
- **Debugging**: Tools for troubleshooting
- **IDE Integration**: Set up your development environment

---

<div align="center">

**🎯 You're Now a Utopia Developer!**

*Build amazing multi-language applications with confidence*

[**🏠 Back to Docs**](README.md) • [**📝 Syntax Reference**](utopia-syntax.md) • [**💡 Examples**](examples.md)

</div> 