// rcalc Web REPL Application

class CalcREPL {
    constructor() {
        this.input = document.getElementById('input');
        this.output = document.getElementById('output');
        this.history = [];
        this.historyIndex = -1;

        this.setupEventListeners();
        this.showWelcome();
    }

    setupEventListeners() {
        this.input.addEventListener('keydown', (e) => this.handleKeydown(e));
        this.input.addEventListener('keyup', (e) => this.handleKeyup(e));
        this.input.focus();
    }

    handleKeydown(e) {
        if (e.key === 'Enter') {
            e.preventDefault();
            this.evaluate();
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            this.historyUp();
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            this.historyDown();
        } else if (e.ctrlKey && e.key === 'l') {
            e.preventDefault();
            this.output.innerHTML = '';
        }
    }

    handleKeyup(e) {
        // Auto-height textarea-like behavior
        if (this.input.scrollHeight > 50) {
            this.input.style.height = this.input.scrollHeight + 'px';
        }
    }

    historyUp() {
        if (this.historyIndex < this.history.length - 1) {
            this.historyIndex++;
            this.input.value = this.history[this.history.length - 1 - this.historyIndex];
        }
    }

    historyDown() {
        if (this.historyIndex > 0) {
            this.historyIndex--;
            this.input.value = this.history[this.history.length - 1 - this.historyIndex];
        } else if (this.historyIndex === 0) {
            this.historyIndex = -1;
            this.input.value = '';
        }
    }

    evaluate() {
        const expr = this.input.value.trim();

        if (!expr) return;

        // Add to history
        this.history.push(expr);
        this.historyIndex = -1;

        // Show expression
        this.addOutput(`<span class="output-expr">&gt; ${this.escapeHtml(expr)}</span>`);

        // Special case: help
        if (expr === 'help' || expr.startsWith('help ')) {
            this.showHelp(expr.substring(5).trim());
            this.input.value = '';
            this.input.focus();
            return;
        }

        // Send to backend
        this.input.disabled = true;
        this.input.placeholder = 'Computing...';

        fetch('/api/calc', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ expression: expr }),
        })
            .then((response) => response.json())
            .then((data) => {
                if (data.success) {
                    this.addOutput(`<span class="output-result">${this.escapeHtml(data.result)}</span>`);
                } else {
                    this.addOutput(
                        `<span class="output-error">Error: ${this.escapeHtml(data.error)}</span>`
                    );
                }
            })
            .catch((error) => {
                this.addOutput(`<span class="output-error">Connection error: ${error.message}</span>`);
            })
            .finally(() => {
                this.input.disabled = false;
                this.input.placeholder =
                    'Enter expression (e.g., 2^100, sin(pi()/6), list(1,2,3))';
                this.input.value = '';
                this.input.focus();
            });
    }

    showWelcome() {
        this.addOutput('<span style="color: #667eea; font-weight: 600;">rcalc Web REPL</span>');
        this.addOutput('');
        this.addOutput(
            '<span>Type <code style="background: #f0f0f0; padding: 2px 6px;">help</code> for documentation.</span>'
        );
        this.addOutput('');
        this.addOutput('<span style="color: #999; font-size: 12px;">Examples:</span>');
        this.addOutput('<span style="color: #999; font-size: 12px;">  2^256             - Big numbers (exact)</span>');
        this.addOutput('<span style="color: #999; font-size: 12px;">  1/3 * 3           - Exact rationals</span>');
        this.addOutput('<span style="color: #999; font-size: 12px;">  sin(pi()/6)       - Math functions</span>');
        this.addOutput('<span style="color: #999; font-size: 12px;">  list(1,2,3,4,5)   - Lists</span>');
        this.addOutput('');
    }

    showHelp(filter) {
        const topics = [
            'intro',
            'usage',
            'builtin',
            'define',
            'statement',
            'expression',
            'operator',
            'variable',
            'number',
            'config',
            'type',
            'list',
            'string',
            'mat',
            'assoc',
            'file',
            'error',
            'resource',
            'mcp',
        ];

        if (!filter) {
            this.addOutput(
                '<span class="output-help">Help topics: ' + topics.join('  ') + '\n\nUsage:\n  help &lt;topic&gt;  — show topic docs (e.g. help intro)\n  help &lt;name&gt;   — search functions (e.g. help sin)</span>'
            );
            return;
        }

        // For now, just show a list of topics
        const matching = topics.filter((t) => t.includes(filter.toLowerCase()));
        if (matching.length > 0) {
            this.addOutput(
                '<span class="output-help">Topics matching "' +
                    filter +
                    '":\n\n' +
                    matching.join('\n') +
                    '\n\nType "help <topic>" to see full documentation.</span>'
            );
        } else {
            this.addOutput(
                '<span class="output-help">No topics or functions found matching "' +
                    filter +
                    '"\n\nAvailable topics: ' +
                    topics.join(', ') +
                    '</span>'
            );
        }
    }

    addOutput(html) {
        const line = document.createElement('div');
        line.className = 'output-line';
        line.innerHTML = html;
        this.output.appendChild(line);
        this.output.scrollTop = this.output.scrollHeight;
    }

    escapeHtml(text) {
        const map = {
            '&': '&amp;',
            '<': '&lt;',
            '>': '&gt;',
            '"': '&quot;',
            "'": '&#039;',
        };
        return text.replace(/[&<>"']/g, (m) => map[m]);
    }
}

// Initialize REPL when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    new CalcREPL();
});
