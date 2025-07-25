# 💡 **Utopia Code Examples**

Comprehensive collection of practical code examples to help you master Utopia's multi-language capabilities. From simple tutorials to complex real-world applications.

---

## 📚 **Example Categories**

### **🎯 [Beginner Examples](#-beginner-examples)**
Start your Utopia journey with simple, clear examples

### **🏗️ [Foundation Examples](#-foundation-examples)**  
Master core programming concepts and patterns

### **🌐 [Multi-Language Examples](#-multi-language-examples)**
Learn cross-language integration techniques

### **🚀 [Advanced Examples](#-advanced-examples)**
Build complex, production-ready applications

### **💼 [Real-World Projects](#-real-world-projects)**
Complete applications demonstrating best practices

---

## 🎯 **Beginner Examples**

### **Hello World Variations**

#### **Basic Hello World**
```utopia
// hello-basic.uto
println("Hello, Utopia!")
println("Your first multi-language compiler!")
```

#### **Interactive Hello World**
```utopia
// hello-interactive.uto
let name = "Developer"
let language = "Utopia"

println("Hello,", name + "!")
println("Welcome to", language, "programming!")

let currentYear = 2024
println("Programming in", currentYear, "is amazing!")
```

#### **Multi-Language Hello World**
```utopia
// hello-multi.uto
@lang python
def create_greeting(name, lang):
    return f"Hello {name}, welcome to {lang}!"

@lang javascript
function formatMessage(greeting) {
    return `🎉 ${greeting} 🚀`;
}

@lang main
let userName = "Alice"
let languageName = "Utopia"

let greeting = py::create_greeting(userName, languageName)
let formatted = js::formatMessage(greeting)

println(formatted)
```

### **Variables and Data Types**

#### **Basic Variables**
```utopia
// variables-basic.uto
// Numbers
let age = 25
let temperature = 98.6
let negative = -42

// Strings
let firstName = "John"
let lastName = 'Doe'
let fullName = firstName + " " + lastName

// Booleans
let isActive = true
let isComplete = false

// Display all variables
println("Age:", age)
println("Temperature:", temperature, "°F")
println("Full Name:", fullName)
println("Status - Active:", isActive, "Complete:", isComplete)
```

#### **Type Conversion Examples**
```utopia
// type-conversion.uto
let numberAsString = "42"
let stringAsNumber = 42
let booleanValue = true

println("Original values:")
println("String:", numberAsString, "Number:", stringAsNumber, "Boolean:", booleanValue)

// Simple conversions (manual for now)
let convertedString = "Number is: " + stringAsNumber
let convertedBoolean = booleanValue ? "Yes" : "No"

println("Converted values:")
println("String conversion:", convertedString)
println("Boolean to text:", convertedBoolean)
```

### **Basic Control Flow**

#### **Conditional Logic**
```utopia
// conditionals.uto
let score = 87
let grade = ""

if (score >= 90) {
    grade = "A"
} else if (score >= 80) {
    grade = "B"
} else if (score >= 70) {
    grade = "C"
} else if (score >= 60) {
    grade = "D"
} else {
    grade = "F"
}

println("Score:", score, "Grade:", grade)

// Nested conditions
let attendance = 95
let canPass = false

if (grade != "F") {
    if (attendance >= 80) {
        canPass = true
        println("Congratulations! You pass the course.")
    } else {
        println("Grade is good, but attendance is too low.")
    }
} else {
    println("Sorry, you need to retake the course.")
}
```

#### **Loops and Iteration**
```utopia
// loops.uto
println("=== FOR LOOP EXAMPLES ===")

// Basic counting
println("Counting 1 to 5:")
for (let i = 1; i <= 5; i++) {
    println("Count:", i)
}

// Counting down
println("\nCountdown from 5:")
for (let i = 5; i > 0; i--) {
    println(i)
}
println("Blast off! 🚀")

// Stepping by 2
println("\nEven numbers 2 to 10:")
for (let i = 2; i <= 10; i += 2) {
    println(i)
}

println("\n=== WHILE LOOP EXAMPLES ===")

// Basic while loop
let count = 0
println("While loop counting to 3:")
while (count < 3) {
    count++
    println("Iteration:", count)
}

// While loop with condition
let number = 1
println("\nPowers of 2 less than 100:")
while (number < 100) {
    println(number)
    number = number * 2
}
```

---

## 🏗️ **Foundation Examples**

### **Functions and Modularity**

#### **Basic Functions**
```utopia
// functions-basic.uto
// Simple function
function greet() {
    println("Hello from a function!")
}

// Function with parameters
function add(a, b) {
    return a + b
}

// Function with multiple parameters
function calculateRectangleArea(length, width) {
    let area = length * width
    return area
}

// Function with conditional logic
function getGrade(score) {
    if (score >= 90) return "A"
    if (score >= 80) return "B"
    if (score >= 70) return "C"
    if (score >= 60) return "D"
    return "F"
}

// Using the functions
greet()

let sum = add(15, 25)
println("15 + 25 =", sum)

let area = calculateRectangleArea(5, 8)
println("Rectangle area:", area, "square units")

let studentGrade = getGrade(85)
println("Student grade:", studentGrade)
```

#### **Advanced Functions**
```utopia
// functions-advanced.uto
// Function that calls other functions
function calculateCircleStats(radius) {
    function calculateArea(r) {
        let pi = 3.14159
        return pi * r * r
    }
    
    function calculateCircumference(r) {
        let pi = 3.14159
        return 2 * pi * r
    }
    
    return {
        radius: radius,
        area: calculateArea(radius),
        circumference: calculateCircumference(radius)
    }
}

// Function with validation
function divide(dividend, divisor) {
    if (divisor == 0) {
        println("Error: Cannot divide by zero!")
        return null
    }
    return dividend / divisor
}

// Recursive function
function factorial(n) {
    if (n <= 1) {
        return 1
    }
    return n * factorial(n - 1)
}

// Function that processes arrays
function findMax(numbers) {
    if (numbers.length == 0) {
        return null
    }
    
    let max = numbers[0]
    for (let i = 1; i < numbers.length; i++) {
        if (numbers[i] > max) {
            max = numbers[i]
        }
    }
    return max
}

// Using advanced functions
let circleStats = calculateCircleStats(5)
println("Circle with radius 5:")
println("  Area:", circleStats.area)
println("  Circumference:", circleStats.circumference)

let result = divide(10, 3)
if (result != null) {
    println("10 ÷ 3 =", result)
}

println("5! =", factorial(5))

let numbers = [3, 7, 2, 9, 1, 8]
println("Maximum in", numbers, "is", findMax(numbers))
```

### **Data Structures**

#### **Working with Arrays**
```utopia
// arrays.uto
// Creating arrays
let fruits = ["apple", "banana", "orange", "grape"]
let numbers = [10, 25, 5, 40, 15, 30]
let mixed = [1, "hello", true, 3.14]

println("=== ARRAY BASICS ===")
println("Fruits:", fruits)
println("First fruit:", fruits[0])
println("Last fruit:", fruits[3])

// Array information
println("Number of fruits:", fruits.length)
println("Numbers array:", numbers)

println("\n=== ARRAY ITERATION ===")
// Loop through array
println("All fruits:")
for (let i = 0; i < fruits.length; i++) {
    println((i + 1) + ".", fruits[i])
}

// Array processing
println("\nNumbers greater than 20:")
for (let i = 0; i < numbers.length; i++) {
    if (numbers[i] > 20) {
        println(numbers[i])
    }
}

// Array statistics
function arrayStats(arr) {
    let sum = 0
    let min = arr[0]
    let max = arr[0]
    
    for (let i = 0; i < arr.length; i++) {
        sum += arr[i]
        if (arr[i] < min) min = arr[i]
        if (arr[i] > max) max = arr[i]
    }
    
    return {
        count: arr.length,
        sum: sum,
        average: sum / arr.length,
        min: min,
        max: max
    }
}

let stats = arrayStats(numbers)
println("\n=== ARRAY STATISTICS ===")
println("Count:", stats.count)
println("Sum:", stats.sum)
println("Average:", stats.average)
println("Min:", stats.min)
println("Max:", stats.max)
```

#### **Working with Objects**
```utopia
// objects.uto
// Creating objects
let person = {
    firstName: "Alice",
    lastName: "Johnson",
    age: 30,
    city: "San Francisco",
    isEmployed: true
}

let car = {
    make: "Toyota",
    model: "Camry",
    year: 2022,
    color: "blue",
    mileage: 15000
}

println("=== OBJECT BASICS ===")
println("Person:", person.firstName, person.lastName)
println("Age:", person.age)
println("Location:", person.city)
println("Employed:", person.isEmployed)

println("\nCar:", car.year, car.make, car.model)
println("Color:", car.color)
println("Mileage:", car.mileage, "miles")

// Object modification
println("\n=== OBJECT MODIFICATION ===")
person.age = 31
person.occupation = "Software Developer"
println("Updated age:", person.age)
println("New occupation:", person.occupation)

// Nested objects
let company = {
    name: "Tech Solutions Inc",
    founded: 2015,
    employees: 250,
    headquarters: {
        street: "123 Innovation Drive",
        city: "San Francisco",
        state: "CA",
        zipCode: "94107"
    },
    departments: ["Engineering", "Sales", "Marketing", "HR"]
}

println("\n=== NESTED OBJECTS ===")
println("Company:", company.name)
println("Founded:", company.founded)
println("Employees:", company.employees)
println("Address:", company.headquarters.street)
println("City:", company.headquarters.city + ",", company.headquarters.state)

println("\nDepartments:")
for (let i = 0; i < company.departments.length; i++) {
    println("-", company.departments[i])
}

// Object methods simulation
function getPersonInfo(person) {
    return person.firstName + " " + person.lastName + " (" + person.age + " years old)"
}

function getCarDescription(car) {
    return car.year + " " + car.color + " " + car.make + " " + car.model
}

println("\n=== OBJECT PROCESSING ===")
println("Person info:", getPersonInfo(person))
println("Car description:", getCarDescription(car))
```

---

## 🌐 **Multi-Language Examples**

### **Basic Cross-Language Integration**

#### **Python + JavaScript Data Processing**
```utopia
// data-processing.uto
@lang python
import json
import math

def analyze_sales_data(sales):
    """Analyze sales data using Python's math capabilities"""
    total = sum(sales)
    count = len(sales)
    average = total / count
    
    # Calculate standard deviation
    variance = sum((x - average) ** 2 for x in sales) / count
    std_dev = math.sqrt(variance)
    
    return {
        'total': total,
        'count': count,
        'average': average,
        'std_dev': std_dev,
        'min': min(sales),
        'max': max(sales)
    }

def categorize_performance(average, std_dev):
    """Categorize sales performance"""
    if average > 1000:
        if std_dev < 200:
            return "Excellent and Consistent"
        else:
            return "Excellent but Variable"
    elif average > 500:
        if std_dev < 150:
            return "Good and Stable"
        else:
            return "Good but Inconsistent"
    else:
        return "Needs Improvement"

@lang javascript
function formatCurrency(amount) {
    return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD'
    }).format(amount);
}

function createSalesReport(analysis, category) {
    const report = {
        title: "Sales Performance Report",
        generatedAt: new Date().toLocaleString(),
        summary: {
            totalSales: formatCurrency(analysis.total),
            averageSale: formatCurrency(analysis.average),
            salesCount: analysis.count,
            performance: category
        },
        statistics: {
            minimum: formatCurrency(analysis.min),
            maximum: formatCurrency(analysis.max),
            standardDeviation: formatCurrency(analysis.std_dev)
        }
    };
    
    return report;
}

function displayReport(report) {
    console.log(`
${report.title}
Generated: ${report.generatedAt}

Summary:
- Total Sales: ${report.summary.totalSales}
- Average Sale: ${report.summary.averageSale}  
- Number of Sales: ${report.summary.salesCount}
- Performance: ${report.summary.performance}

Statistics:
- Minimum Sale: ${report.statistics.minimum}
- Maximum Sale: ${report.statistics.maximum}
- Standard Deviation: ${report.statistics.standardDeviation}
    `);
}

@lang main
// Sample sales data
let salesData = [850, 1200, 950, 1100, 750, 1350, 900, 1050, 1250, 800]

println("📊 Sales Data Analysis")
println("Raw data:", salesData)

// Use Python for statistical analysis
let analysis = py::analyze_sales_data(salesData)
let category = py::categorize_performance(analysis.average, analysis.std_dev)

// Use JavaScript for formatting and reporting
let report = js::createSalesReport(analysis, category)

println("\n" + "=".repeat(50))
println("SALES PERFORMANCE REPORT")
println("Generated:", report.generatedAt)
println("=".repeat(50))

println("\nSUMMARY:")
println("Total Sales:", report.summary.totalSales)
println("Average Sale:", report.summary.averageSale)
println("Number of Sales:", report.summary.salesCount)
println("Performance:", report.summary.performance)

println("\nSTATISTICS:")
println("Minimum Sale:", report.statistics.minimum)
println("Maximum Sale:", report.statistics.maximum)
println("Standard Deviation:", report.statistics.standardDeviation)
```

#### **Text Processing Pipeline**
```utopia
// text-processing.uto
@lang python
import re
from collections import Counter

def clean_text(text):
    """Clean and normalize text"""
    # Convert to lowercase
    text = text.lower()
    
    # Remove punctuation except spaces
    text = re.sub(r'[^\w\s]', '', text)
    
    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text).strip()
    
    return text

def analyze_text(text):
    """Perform text analysis"""
    words = text.split()
    word_count = Counter(words)
    
    return {
        'word_count': len(words),
        'unique_words': len(word_count),
        'most_common': word_count.most_common(5),
        'average_word_length': sum(len(word) for word in words) / len(words) if words else 0
    }

def extract_statistics(text):
    """Extract detailed text statistics"""
    sentences = text.count('.') + text.count('!') + text.count('?')
    paragraphs = text.count('\n\n') + 1
    
    return {
        'characters': len(text),
        'characters_no_spaces': len(text.replace(' ', '')),
        'sentences': sentences if sentences > 0 else 1,
        'paragraphs': paragraphs
    }

@lang javascript
function formatTextReport(analysis, statistics, originalText) {
    const readingTime = Math.ceil(analysis.word_count / 200); // 200 WPM average
    const readingLevel = analysis.average_word_length > 5 ? 'Advanced' : 
                        analysis.average_word_length > 4 ? 'Intermediate' : 'Basic';
    
    return {
        original: {
            preview: originalText.substring(0, 100) + (originalText.length > 100 ? '...' : ''),
            length: originalText.length
        },
        metrics: {
            words: analysis.word_count,
            uniqueWords: analysis.unique_words,
            characters: statistics.characters,
            sentences: statistics.sentences,
            paragraphs: statistics.paragraphs
        },
        readability: {
            averageWordLength: analysis.average_word_length.toFixed(2),
            readingLevel: readingLevel,
            estimatedReadingTime: `${readingTime} minute${readingTime !== 1 ? 's' : ''}`
        },
        topWords: analysis.most_common
    };
}

function displayTextAnalysis(report) {
    console.log(`
TEXT ANALYSIS REPORT
${'='.repeat(40)}

Original Text Preview:
"${report.original.preview}"

METRICS:
- Words: ${report.metrics.words}
- Unique Words: ${report.metrics.uniqueWords}
- Characters: ${report.metrics.characters}
- Sentences: ${report.metrics.sentences}
- Paragraphs: ${report.metrics.paragraphs}

READABILITY:
- Average Word Length: ${report.readability.averageWordLength} characters
- Reading Level: ${report.readability.readingLevel}
- Estimated Reading Time: ${report.readability.estimatedReadingTime}

TOP 5 MOST COMMON WORDS:
${report.topWords.map((word, index) => `${index + 1}. "${word[0]}" (${word[1]} times)`).join('\n')}
    `);
}

@lang main
let sampleText = `
The quick brown fox jumps over the lazy dog. This is a classic pangram used in typing practice.
Programming is both an art and a science. It requires creativity, logic, and patience.

The Utopia language allows developers to write code once and compile to multiple targets.
This multi-language approach opens up new possibilities for software development.
Python excels at data analysis, while JavaScript dominates web development.
By combining them, we get the best of both worlds!
`

println("📝 Text Processing Pipeline")
println("Analyzing sample text...")

// Step 1: Clean text with Python
let cleanedText = py::clean_text(sampleText)
println("\n✅ Text cleaned")

// Step 2: Analyze text with Python
let analysis = py::analyze_text(cleanedText)
let statistics = py::extract_statistics(sampleText)
println("✅ Analysis complete")

// Step 3: Format report with JavaScript
let report = js::formatTextReport(analysis, statistics, sampleText)
println("✅ Report generated")

// Display results
println("\n" + "=".repeat(50))
println("TEXT ANALYSIS RESULTS")
println("=".repeat(50))

println("\nOriginal Text Preview:")
println('"' + report.original.preview + '"')

println("\nMETRICS:")
println("Words:", report.metrics.words)
println("Unique Words:", report.metrics.uniqueWords)
println("Characters:", report.metrics.characters)
println("Sentences:", report.metrics.sentences)
println("Paragraphs:", report.metrics.paragraphs)

println("\nREADABILITY:")
println("Average Word Length:", report.readability.averageWordLength, "characters")
println("Reading Level:", report.readability.readingLevel)
println("Estimated Reading Time:", report.readability.estimatedReadingTime)

println("\nTOP 5 MOST COMMON WORDS:")
for (let i = 0; i < report.topWords.length; i++) {
    let word = report.topWords[i]
    println((i + 1) + '. "' + word[0] + '" (' + word[1] + ' times)')
}
```

### **Advanced Cross-Language Patterns**

#### **Web Scraping and Data Visualization**
```utopia
// web-scraping.uto
@lang python
import json
import random
from datetime import datetime, timedelta

def simulate_web_scraping(url):
    """Simulate web scraping (returns mock data)"""
    # In real implementation, would use requests/beautifulsoup
    mock_data = {
        'url': url,
        'scraped_at': datetime.now().isoformat(),
        'products': []
    }
    
    # Generate mock product data
    product_names = ['Laptop', 'Phone', 'Tablet', 'Monitor', 'Keyboard', 'Mouse']
    brands = ['TechCorp', 'InnoTech', 'ProDevice', 'SmartTech']
    
    for i in range(random.randint(5, 10)):
        product = {
            'id': f'prod_{i+1}',
            'name': f'{random.choice(brands)} {random.choice(product_names)}',
            'price': round(random.uniform(50, 1500), 2),
            'rating': round(random.uniform(3.5, 5.0), 1),
            'reviews': random.randint(10, 500),
            'in_stock': random.choice([True, False])
        }
        mock_data['products'].append(product)
    
    return mock_data

def analyze_product_data(data):
    """Analyze scraped product data"""
    products = data['products']
    
    if not products:
        return {'error': 'No products found'}
    
    prices = [p['price'] for p in products]
    ratings = [p['rating'] for p in products]
    
    analysis = {
        'total_products': len(products),
        'price_analysis': {
            'min': min(prices),
            'max': max(prices),
            'avg': sum(prices) / len(prices),
            'median': sorted(prices)[len(prices)//2]
        },
        'rating_analysis': {
            'avg_rating': sum(ratings) / len(ratings),
            'high_rated_count': len([r for r in ratings if r >= 4.5])
        },
        'availability': {
            'in_stock': len([p for p in products if p['in_stock']]),
            'out_of_stock': len([p for p in products if not p['in_stock']])
        }
    }
    
    return analysis

def filter_products(data, min_price=0, max_price=float('inf'), min_rating=0):
    """Filter products based on criteria"""
    filtered = []
    for product in data['products']:
        if (min_price <= product['price'] <= max_price and 
            product['rating'] >= min_rating):
            filtered.append(product)
    
    return filtered

@lang javascript
function createProductVisualization(analysis) {
    // Simulate chart creation for price distribution
    const priceRanges = [
        { range: '$0-$100', count: 0 },
        { range: '$100-$500', count: 0 },
        { range: '$500-$1000', count: 0 },
        { range: '$1000+', count: 0 }
    ];
    
    // In real implementation, would categorize actual products
    // For demo, create mock distribution
    priceRanges[0].count = Math.floor(analysis.total_products * 0.2);
    priceRanges[1].count = Math.floor(analysis.total_products * 0.4);
    priceRanges[2].count = Math.floor(analysis.total_products * 0.3);
    priceRanges[3].count = analysis.total_products - 
                          priceRanges[0].count - 
                          priceRanges[1].count - 
                          priceRanges[2].count;
    
    return {
        type: 'bar_chart',
        title: 'Price Distribution',
        data: priceRanges,
        config: {
            xAxis: 'Price Range',
            yAxis: 'Number of Products',
            colors: ['#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4']
        }
    };
}

function generateDashboard(scrapedData, analysis, chart, filteredProducts) {
    const dashboard = {
        title: 'Product Analysis Dashboard',
        timestamp: new Date().toISOString(),
        source: scrapedData.url,
        overview: {
            totalProducts: analysis.total_products,
            priceRange: `$${analysis.price_analysis.min} - $${analysis.price_analysis.max}`,
            averagePrice: `$${analysis.price_analysis.avg.toFixed(2)}`,
            averageRating: `${analysis.rating_analysis.avg_rating.toFixed(1)}/5.0`,
            inStock: `${analysis.availability.in_stock}/${analysis.total_products}`
        },
        insights: [
            `Average product price is $${analysis.price_analysis.avg.toFixed(2)}`,
            `${analysis.rating_analysis.high_rated_count} products have 4.5+ star ratings`,
            `${((analysis.availability.in_stock / analysis.total_products) * 100).toFixed(1)}% of products are in stock`,
            `Price range spans $${(analysis.price_analysis.max - analysis.price_analysis.min).toFixed(2)}`
        ],
        visualization: chart,
        topProducts: filteredProducts.slice(0, 3)
    };
    
    return dashboard;
}

@lang main
println("🕷️  Web Scraping and Analysis Demo")
println("=" * 40)

// Step 1: Scrape data (simulated)
let targetUrl = "https://example-shop.com/products"
println("Scraping products from:", targetUrl)
let scrapedData = py::simulate_web_scraping(targetUrl)
println("✅ Scraped", scrapedData.products.length, "products")

// Step 2: Analyze the data
println("\nAnalyzing product data...")
let analysis = py::analyze_product_data(scrapedData)
println("✅ Analysis complete")

// Step 3: Filter high-quality products
println("\nFiltering for premium products...")
let premiumProducts = py::filter_products(scrapedData, 200, 1500, 4.0)
println("✅ Found", premiumProducts.length, "premium products")

// Step 4: Create visualization
println("\nGenerating visualization...")
let chart = js::createProductVisualization(analysis)
println("✅ Chart created:", chart.title)

// Step 5: Generate dashboard
println("\nCreating dashboard...")
let dashboard = js::generateDashboard(scrapedData, analysis, chart, premiumProducts)
println("✅ Dashboard ready")

// Display results
println("\n" + "=" * 50)
println("PRODUCT ANALYSIS DASHBOARD")
println("Generated:", dashboard.timestamp)
println("Source:", dashboard.source)
println("=" * 50)

println("\nOVERVIEW:")
println("Total Products:", dashboard.overview.totalProducts)
println("Price Range:", dashboard.overview.priceRange)
println("Average Price:", dashboard.overview.averagePrice)
println("Average Rating:", dashboard.overview.averageRating)
println("In Stock:", dashboard.overview.inStock)

println("\nKEY INSIGHTS:")
for (let i = 0; i < dashboard.insights.length; i++) {
    println("•", dashboard.insights[i])
}

println("\nVISUALIZATION:", dashboard.visualization.title)
println("Chart Type:", dashboard.visualization.type)

println("\nTOP PREMIUM PRODUCTS:")
for (let i = 0; i < dashboard.topProducts.length; i++) {
    let product = dashboard.topProducts[i]
    println((i + 1) + ".", product.name)
    println("   Price: $" + product.price + " | Rating:", product.rating + "/5")
    println("   Reviews:", product.reviews + " | In Stock:", product.in_stock ? "Yes" : "No")
}
```

---

## 🚀 **Advanced Examples**

### **RESTful API Client**
```utopia
// api-client.uto
@lang python
import json
import time
from datetime import datetime

class APIClient:
    def __init__(self, base_url, api_key=None):
        self.base_url = base_url.rstrip('/')
        self.api_key = api_key
        self.request_count = 0
        self.rate_limit = 100  # requests per hour
        
    def simulate_request(self, endpoint, method='GET', data=None):
        """Simulate HTTP request (would use requests library in real implementation)"""
        self.request_count += 1
        
        # Simulate different response times
        import random
        response_time = random.uniform(0.1, 0.5)
        time.sleep(response_time / 10)  # Shortened for demo
        
        # Mock responses based on endpoint
        if endpoint == '/users':
            return {
                'status_code': 200,
                'data': [
                    {'id': 1, 'name': 'Alice Johnson', 'email': 'alice@example.com'},
                    {'id': 2, 'name': 'Bob Smith', 'email': 'bob@example.com'},
                    {'id': 3, 'name': 'Carol Davis', 'email': 'carol@example.com'}
                ],
                'response_time': response_time
            }
        elif endpoint.startswith('/users/'):
            user_id = int(endpoint.split('/')[-1])
            return {
                'status_code': 200,
                'data': {
                    'id': user_id,
                    'name': f'User {user_id}',
                    'email': f'user{user_id}@example.com',
                    'profile': {
                        'age': 25 + user_id,
                        'city': ['New York', 'London', 'Tokyo'][user_id % 3]
                    }
                },
                'response_time': response_time
            }
        elif endpoint == '/stats':
            return {
                'status_code': 200,
                'data': {
                    'total_users': 1250,
                    'active_today': 89,
                    'server_uptime': '99.9%',
                    'response_time_avg': '150ms'
                },
                'response_time': response_time
            }
        else:
            return {
                'status_code': 404,
                'error': 'Endpoint not found',
                'response_time': response_time
            }
    
    def get_users(self):
        """Get all users"""
        return self.simulate_request('/users')
    
    def get_user(self, user_id):
        """Get specific user"""
        return self.simulate_request(f'/users/{user_id}')
    
    def get_statistics(self):
        """Get API statistics"""
        return self.simulate_request('/stats')
    
    def get_request_stats(self):
        """Get client statistics"""
        return {
            'requests_made': self.request_count,
            'remaining_quota': self.rate_limit - self.request_count
        }

@lang javascript
class ResponseProcessor {
    constructor() {
        this.processedCount = 0;
    }
    
    processUserList(response) {
        if (response.status_code !== 200) {
            return { error: `API Error: ${response.status_code}` };
        }
        
        const users = response.data.map(user => ({
            id: user.id,
            displayName: user.name,
            email: user.email,
            emailDomain: user.email.split('@')[1]
        }));
        
        this.processedCount++;
        
        return {
            success: true,
            users: users,
            metadata: {
                count: users.length,
                responseTime: `${(response.response_time * 1000).toFixed(0)}ms`,
                processedAt: new Date().toISOString()
            }
        };
    }
    
    processUserDetail(response) {
        if (response.status_code !== 200) {
            return { error: `API Error: ${response.status_code}` };
        }
        
        const user = response.data;
        this.processedCount++;
        
        return {
            success: true,
            user: {
                id: user.id,
                name: user.name,
                email: user.email,
                age: user.profile.age,
                location: user.profile.city,
                emailDomain: user.email.split('@')[1]
            },
            metadata: {
                responseTime: `${(response.response_time * 1000).toFixed(0)}ms`,
                processedAt: new Date().toISOString()
            }
        };
    }
    
    createSummaryReport(userListResult, userDetails, stats) {
        const report = {
            title: 'API Client Summary Report',
            generatedAt: new Date().toISOString(),
            overview: {
                totalUsers: userListResult.metadata.count,
                detailsFetched: userDetails.length,
                successfulRequests: userDetails.filter(d => d.success).length,
                averageResponseTime: this.calculateAverageResponseTime(userDetails)
            },
            userSummary: userListResult.users.map(user => ({
                id: user.id,
                name: user.displayName,
                domain: user.emailDomain
            })),
            statistics: stats
        };
        
        return report;
    }
    
    calculateAverageResponseTime(responses) {
        const times = responses
            .filter(r => r.success)
            .map(r => parseFloat(r.metadata.responseTime.replace('ms', '')));
        
        const average = times.reduce((sum, time) => sum + time, 0) / times.length;
        return `${average.toFixed(0)}ms`;
    }
}

@lang main
println("🌐 RESTful API Client Demo")
println("=" * 40)

// Initialize API client
let apiClient = py::APIClient("https://api.example.com", "demo-key-123")
let processor = js::ResponseProcessor()

// Step 1: Get all users
println("Fetching user list...")
let usersResponse = apiClient.get_users()
let processedUsers = processor.processUserList(usersResponse)

if (processedUsers.success) {
    println("✅ Retrieved", processedUsers.metadata.count, "users")
    println("   Response time:", processedUsers.metadata.responseTime)
} else {
    println("❌ Error:", processedUsers.error)
}

// Step 2: Get detailed info for each user
println("\nFetching user details...")
let userDetails = []

for (let i = 0; i < processedUsers.users.length; i++) {
    let userId = processedUsers.users[i].id
    println("  Fetching details for user", userId + "...")
    
    let userResponse = apiClient.get_user(userId)
    let processedDetail = processor.processUserDetail(userResponse)
    userDetails.push(processedDetail)
    
    if (processedDetail.success) {
        println("    ✅ User:", processedDetail.user.name)
    } else {
        println("    ❌ Error:", processedDetail.error)
    }
}

// Step 3: Get API statistics
println("\nFetching API statistics...")
let statsResponse = apiClient.get_statistics()
let clientStats = apiClient.get_request_stats()

// Step 4: Generate summary report
println("\nGenerating summary report...")
let report = processor.createSummaryReport(processedUsers, userDetails, {
    api: statsResponse.data,
    client: clientStats
})

// Display results
println("\n" + "=" * 50)
println("API CLIENT SUMMARY REPORT")
println("Generated:", report.generatedAt)
println("=" * 50)

println("\nOVERVIEW:")
println("Total Users:", report.overview.totalUsers)
println("Details Fetched:", report.overview.detailsFetched)
println("Successful Requests:", report.overview.successfulRequests)
println("Average Response Time:", report.overview.averageResponseTime)

println("\nUSER SUMMARY:")
for (let i = 0; i < report.userSummary.length; i++) {
    let user = report.userSummary[i]
    println((i + 1) + ".", user.name, "(" + user.domain + ")")
}

println("\nAPI STATISTICS:")
println("Total Users in System:", report.statistics.api.total_users)
println("Active Today:", report.statistics.api.active_today)
println("Server Uptime:", report.statistics.api.server_uptime)
println("Average Response Time:", report.statistics.api.response_time_avg)

println("\nCLIENT STATISTICS:")
println("Requests Made:", report.statistics.client.requests_made)
println("Remaining Quota:", report.statistics.client.remaining_quota)

println("\nDETAILED USER INFO:")
for (let i = 0; i < userDetails.length; i++) {
    if (userDetails[i].success) {
        let user = userDetails[i].user
        println((i + 1) + ".", user.name)
        println("   Email:", user.email)
        println("   Age:", user.age + ", Location:", user.location)
        println("   Response time:", userDetails[i].metadata.responseTime)
    }
}
```

---

## 💼 **Real-World Projects**

### **Task Management System**
```utopia
// task-manager.uto
@lang python
from datetime import datetime, timedelta
import json

class TaskManager:
    def __init__(self):
        self.tasks = {}
        self.next_id = 1
        self.categories = ['Work', 'Personal', 'Shopping', 'Health', 'Learning']
        
    def create_task(self, title, description='', category='Personal', priority='Medium', due_date=None):
        """Create a new task"""
        task = {
            'id': self.next_id,
            'title': title,
            'description': description,
            'category': category,
            'priority': priority,
            'status': 'pending',
            'due_date': due_date,
            'created_at': datetime.now().isoformat(),
            'completed_at': None,
            'subtasks': []
        }
        
        self.tasks[self.next_id] = task
        self.next_id += 1
        return task
    
    def update_task(self, task_id, **kwargs):
        """Update an existing task"""
        if task_id not in self.tasks:
            return {'error': 'Task not found'}
        
        task = self.tasks[task_id]
        for key, value in kwargs.items():
            if key in task:
                task[key] = value
        
        return task
    
    def complete_task(self, task_id):
        """Mark task as completed"""
        if task_id not in self.tasks:
            return {'error': 'Task not found'}
        
        task = self.tasks[task_id]
        task['status'] = 'completed'
        task['completed_at'] = datetime.now().isoformat()
        return task
    
    def get_tasks_by_status(self, status):
        """Get tasks filtered by status"""
        return [task for task in self.tasks.values() if task['status'] == status]
    
    def get_tasks_by_category(self, category):
        """Get tasks filtered by category"""
        return [task for task in self.tasks.values() if task['category'] == category]
    
    def get_overdue_tasks(self):
        """Get tasks that are overdue"""
        overdue = []
        now = datetime.now()
        
        for task in self.tasks.values():
            if (task['status'] == 'pending' and 
                task['due_date'] and 
                datetime.fromisoformat(task['due_date']) < now):
                overdue.append(task)
        
        return overdue
    
    def get_statistics(self):
        """Get task statistics"""
        total_tasks = len(self.tasks)
        completed_tasks = len(self.get_tasks_by_status('completed'))
        pending_tasks = len(self.get_tasks_by_status('pending'))
        overdue_tasks = len(self.get_overdue_tasks())
        
        # Category breakdown
        category_stats = {}
        for category in self.categories:
            category_stats[category] = len(self.get_tasks_by_category(category))
        
        return {
            'total_tasks': total_tasks,
            'completed_tasks': completed_tasks,
            'pending_tasks': pending_tasks,
            'overdue_tasks': overdue_tasks,
            'completion_rate': (completed_tasks / total_tasks * 100) if total_tasks > 0 else 0,
            'category_breakdown': category_stats
        }

@lang javascript
class TaskDisplay {
    constructor() {
        this.displayOptions = {
            showCompleted: true,
            sortBy: 'created_at',
            groupBy: 'category'
        };
    }
    
    formatTask(task) {
        const dueDate = task.due_date ? new Date(task.due_date).toLocaleDateString() : 'No due date';
        const createdDate = new Date(task.created_at).toLocaleDateString();
        
        const priorityEmoji = {
            'High': '🔴',
            'Medium': '🟡',
            'Low': '🟢'
        };
        
        const statusEmoji = {
            'pending': '⏳',
            'completed': '✅',
            'in_progress': '🔄'
        };
        
        return {
            id: task.id,
            title: task.title,
            description: task.description,
            category: task.category,
            priority: `${priorityEmoji[task.priority] || '⚪'} ${task.priority}`,
            status: `${statusEmoji[task.status] || '❓'} ${task.status}`,
            dueDate: dueDate,
            createdDate: createdDate,
            isOverdue: task.due_date && new Date(task.due_date) < new Date() && task.status !== 'completed'
        };
    }
    
    createTaskList(tasks) {
        const formattedTasks = tasks.map(task => this.formatTask(task));
        
        // Group by category
        const grouped = {};
        formattedTasks.forEach(task => {
            if (!grouped[task.category]) {
                grouped[task.category] = [];
            }
            grouped[task.category].push(task);
        });
        
        return {
            allTasks: formattedTasks,
            groupedByCategory: grouped,
            totalCount: formattedTasks.length
        };
    }
    
    createDashboard(stats, taskList) {
        const dashboard = {
            title: 'Task Management Dashboard',
            timestamp: new Date().toISOString(),
            summary: {
                totalTasks: stats.total_tasks,
                completedTasks: stats.completed_tasks,
                pendingTasks: stats.pending_tasks,
                overdueTasks: stats.overdue_tasks,
                completionRate: `${stats.completion_rate.toFixed(1)}%`
            },
            insights: [
                `You have ${stats.pending_tasks} pending tasks`,
                stats.overdue_tasks > 0 ? 
                    `⚠️ ${stats.overdue_tasks} tasks are overdue` : 
                    '✅ No overdue tasks',
                `Your completion rate is ${stats.completion_rate.toFixed(1)}%`,
                `Most tasks are in: ${Object.keys(stats.category_breakdown).reduce((a, b) => 
                    stats.category_breakdown[a] > stats.category_breakdown[b] ? a : b)}`
            ],
            categoryBreakdown: stats.category_breakdown,
            recentTasks: taskList.allTasks.slice(-5),
            urgentTasks: taskList.allTasks.filter(task => task.isOverdue)
        };
        
        return dashboard;
    }
    
    generateProductivityReport(tasks, stats) {
        const completedThisWeek = tasks.filter(task => {
            if (task.status !== 'completed' || !task.completed_at) return false;
            const completedDate = new Date(task.completed_at);
            const weekAgo = new Date();
            weekAgo.setDate(weekAgo.getDate() - 7);
            return completedDate > weekAgo;
        });
        
        return {
            weeklyCompleted: completedThisWeek.length,
            weeklyProductivity: completedThisWeek.length > 5 ? 'High' : 
                               completedThisWeek.length > 2 ? 'Medium' : 'Low',
            recommendations: this.generateRecommendations(stats)
        };
    }
    
    generateRecommendations(stats) {
        const recommendations = [];
        
        if (stats.overdue_tasks > 0) {
            recommendations.push('Focus on completing overdue tasks first');
        }
        
        if (stats.completion_rate < 50) {
            recommendations.push('Consider breaking large tasks into smaller ones');
        }
        
        if (stats.pending_tasks > 10) {
            recommendations.push('You might be taking on too many tasks at once');
        }
        
        if (stats.completion_rate > 80) {
            recommendations.push('Great job! Keep up the excellent work');
        }
        
        return recommendations.length > 0 ? recommendations : ['Keep being productive!'];
    }
}

@lang main
println("📋 Task Management System Demo")
println("=" * 40)

// Initialize task manager
let taskManager = py::TaskManager()
let display = js::TaskDisplay()

// Create sample tasks
println("Creating sample tasks...")

let tasks = [
    {title: "Review project proposal", category: "Work", priority: "High", due_date: "2024-01-15T10:00:00"},
    {title: "Buy groceries", category: "Shopping", priority: "Medium", due_date: "2024-01-12T18:00:00"},
    {title: "Finish Python course", category: "Learning", priority: "Medium", due_date: "2024-01-20T23:59:59"},
    {title: "Doctor appointment", category: "Health", priority: "High", due_date: "2024-01-14T14:30:00"},
    {title: "Plan weekend trip", category: "Personal", priority: "Low"},
    {title: "Update resume", category: "Work", priority: "Medium"},
    {title: "Call family", category: "Personal", priority: "Medium"},
    {title: "Gym workout", category: "Health", priority: "Low", due_date: "2024-01-13T07:00:00"}
]

for (let i = 0; i < tasks.length; i++) {
    let task = tasks[i]
    let created = taskManager.create_task(
        task.title, 
        "", 
        task.category, 
        task.priority, 
        task.due_date
    )
    println("✅ Created:", created.title)
}

// Complete some tasks
println("\nCompleting some tasks...")
taskManager.complete_task(2)  // Buy groceries
taskManager.complete_task(7)  // Call family
println("✅ Marked 2 tasks as completed")

// Get statistics
println("\nAnalyzing tasks...")
let stats = taskManager.get_statistics()
let allTasks = Object.values(taskManager.tasks)
let taskList = display.createTaskList(allTasks)

// Create dashboard
let dashboard = display.createDashboard(stats, taskList)
let productivityReport = display.generateProductivityReport(allTasks, stats)

// Display results
println("\n" + "=" * 50)
println("TASK MANAGEMENT DASHBOARD")
println("Generated:", dashboard.timestamp)
println("=" * 50)

println("\nSUMMARY:")
println("Total Tasks:", dashboard.summary.totalTasks)
println("Completed:", dashboard.summary.completedTasks)
println("Pending:", dashboard.summary.pendingTasks)
println("Overdue:", dashboard.summary.overdueTasks)
println("Completion Rate:", dashboard.summary.completionRate)

println("\nKEY INSIGHTS:")
for (let i = 0; i < dashboard.insights.length; i++) {
    println("•", dashboard.insights[i])
}

println("\nCATEGORY BREAKDOWN:")
let categories = Object.keys(dashboard.categoryBreakdown)
for (let i = 0; i < categories.length; i++) {
    let category = categories[i]
    let count = dashboard.categoryBreakdown[category]
    println(category + ":", count, "tasks")
}

println("\nURGENT/OVERDUE TASKS:")
if (dashboard.urgentTasks.length > 0) {
    for (let i = 0; i < dashboard.urgentTasks.length; i++) {
        let task = dashboard.urgentTasks[i]
        println("⚠️", task.title, "- Due:", task.dueDate)
    }
} else {
    println("✅ No overdue tasks!")
}

println("\nRECENT TASKS:")
for (let i = 0; i < dashboard.recentTasks.length; i++) {
    let task = dashboard.recentTasks[i]
    println("•", task.title, "-", task.status, task.priority)
}

println("\nPRODUCTIVITY REPORT:")
println("Tasks completed this week:", productivityReport.weeklyCompleted)
println("Productivity level:", productivityReport.weeklyProductivity)

println("\nRECOMMENDATIONS:")
for (let i = 0; i < productivityReport.recommendations.length; i++) {
    println("💡", productivityReport.recommendations[i])
}

println("\nTASKS BY CATEGORY:")
let groupedTasks = taskList.groupedByCategory
let categoryNames = Object.keys(groupedTasks)
for (let i = 0; i < categoryNames.length; i++) {
    let categoryName = categoryNames[i]
    let categoryTasks = groupedTasks[categoryName]
    println("\n" + categoryName.toUpperCase() + ":")
    for (let j = 0; j < categoryTasks.length; j++) {
        let task = categoryTasks[j]
        let overdueFlag = task.isOverdue ? " ⚠️ OVERDUE" : ""
        println("  " + (j + 1) + ".", task.title, "-", task.status, task.priority + overdueFlag)
    }
}
```

---

## 🎯 **Running the Examples**

### **Quick Setup**
```bash
# Clone and build Utopia
git clone https://github.com/WinsonEdwards/Utopia.git
cd Utopia/utopia-rs
cargo build --release
cargo install --path .
```

### **Run Examples**
```bash
# Beginner examples
utopia run examples/hello-basic.uto
utopia run examples/variables-basic.uto
utopia run examples/control-flow.uto

# Foundation examples  
utopia run examples/functions-basic.uto
utopia run examples/data-structures.uto

# Multi-language examples
utopia run examples/data-processing.uto
utopia run examples/text-processing.uto

# Advanced examples
utopia run examples/api-client.uto
utopia run examples/task-manager.uto
```

### **Compile to Specific Languages**
```bash
# Compile to Python
utopia compile examples/data-processing.uto --target python

# Compile to JavaScript
utopia compile examples/web-scraping.uto --target javascript

# Compile to TypeScript
utopia compile examples/api-client.uto --target typescript
```

---

## 🏆 **Next Steps**

### **📚 Continue Learning**
- **[Syntax Reference](utopia-syntax.md)** - Complete language syntax
- **[Language Guide](utopia-language-guide.md)** - Step-by-step tutorials
- **[CLI Reference](cli-reference.md)** - All compiler commands

### **🛠️ Build Your Own Projects**
1. **Start Simple** - Pick a beginner example and modify it
2. **Add Features** - Extend examples with new functionality  
3. **Mix Languages** - Combine Python data processing with JavaScript UI
4. **Share & Learn** - Contribute your examples to the community

### **🤝 Get Involved**
- **GitHub Issues** - Report bugs or request features
- **Discussions** - Ask questions and share ideas
- **Contributing** - Submit pull requests with improvements

---

<div align="center">

**🎯 Master Utopia by Example!**

*Learn by doing with practical, real-world code samples*

[**🏠 Back to Docs**](README.md) • [**📝 Syntax Guide**](utopia-syntax.md) • [**🚀 Language Guide**](utopia-language-guide.md)

</div> 