// ==========================================================================
// API Configuration and Client
// ==========================================================================

// API Base Configuration
const API_CONFIG = {
    baseURL: process.env.REACT_APP_API_BASE_URL || 'http://localhost:8080',
    timeout: 300000, // 300 seconds
    headers: {
        'Content-Type': 'application/json',
    }
};

// API Client Class
class ApiClient {
    constructor(config = API_CONFIG) {
        this.baseURL = config.baseURL;
        this.timeout = config.timeout;
        this.defaultHeaders = config.headers;
    }

    // Generic request method
    async request(endpoint, options = {}) {
        const url = `${this.baseURL}${endpoint}`;
        const config = {
            timeout: this.timeout,
            headers: {
                ...this.defaultHeaders,
                ...options.headers
            },
            ...options
        };

        try {
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), this.timeout);

            const response = await fetch(url, {
                ...config,
                signal: controller.signal
            });

            clearTimeout(timeoutId);

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const contentType = response.headers.get('Content-Type');
            if (contentType && contentType.includes('application/json')) {
                return await response.json();
            }
            
            return await response.text();
        } catch (error) {
            if (error.name === 'AbortError') {
                throw new Error('Request timeout');
            }
            throw error;
        }
    }

    // GET method
    async get(endpoint, params = {}) {
        const searchParams = new URLSearchParams(params);
        const queryString = searchParams.toString();
        const url = queryString ? `${endpoint}?${queryString}` : endpoint;
        
        return this.request(url, {
            method: 'GET'
        });
    }

    // POST method
    async post(endpoint, data = null) {
        return this.request(endpoint, {
            method: 'POST',
            body: data ? JSON.stringify(data) : null
        });
    }

    // PUT method
    async put(endpoint, data = null) {
        return this.request(endpoint, {
            method: 'PUT',
            body: data ? JSON.stringify(data) : null
        });
    }

    // DELETE method
    async delete(endpoint) {
        return this.request(endpoint, {
            method: 'DELETE'
        });
    }

    // PATCH method
    async patch(endpoint, data = null) {
        return this.request(endpoint, {
            method: 'PATCH',
            body: data ? JSON.stringify(data) : null
        });
    }
}

// Create API client instance
const apiClient = new ApiClient();

// ==========================================================================
// Academic Papers API
// ==========================================================================

export const academicPapersApi = {
    // Get all academic papers
    getAll: async (params = {}) => {
        return apiClient.get('/api/v1/academic-paper/all', params);
    },

    // Get academic paper by ID
    getById: async (id) => {
        return apiClient.get(`/api/v1/academic-paper/paper?paper_id=${id}`);
    },

    // Create new academic paper
    create: async (paperData) => {
        return apiClient.post('/api/v1/academic-paper/add', paperData);
    },

    // Update academic paper
    update: async (id, paperData) => {
        return apiClient.put(`/api/v1/academic-paper/${id}`, paperData);
    },

    // Delete academic paper
    delete: async (id) => {
        return apiClient.delete(`/api/v1/academic-paper/${id}`);
    },

    // Search academic papers
    search: async (query, params = {}) => {
        return apiClient.get('/api/v1/academic-paper/search', { q: query, ...params });
    },

    // Create academic paper with SSE (Server-Sent Events)
    createWithSSE: (paperData, onProgress, onError, onComplete) => {
        const url = `${API_CONFIG.baseURL}/api/v1/academic-paper/add-sse`;
        
        return new Promise((resolve, reject) => {
            const eventSource = new EventSource(url + '?' + new URLSearchParams({
                title: paperData.title,
                pdf_url: paperData.pdf_url
            }));

            eventSource.onmessage = (event) => {
                try {
                    const data = JSON.parse(event.data);
                    console.log('SSE data received:', data);
                    
                    // If progress is 100, consider it complete
                    if (data.progress === 100) {
                        eventSource.close();
                        if (onComplete) {
                            onComplete(data);
                        }
                        resolve(data);
                    } else {
                        // Only call onProgress for non-complete updates
                        if (onProgress) {
                            onProgress(data);
                        }
                    }
                } catch (error) {
                    console.error('Error parsing SSE data:', error);
                    eventSource.close();
                    if (onError) {
                        onError(error);
                    }
                    reject(error);
                }
            };

            eventSource.onerror = (error) => {
                console.error('SSE connection error:', error);
                eventSource.close();
                if (onError) {
                    onError(error);
                }
                reject(error);
            };

            // Return a cleanup function
            return () => {
                eventSource.close();
            };
        });
    }
};

// ==========================================================================
// Web Articles API
// ==========================================================================

export const webArticlesApi = {
    // Get all web articles
    getAll: async (params = {}) => {
        return apiClient.get('/api/web-articles', params);
    },

    // Get web article by ID
    getById: async (id) => {
        return apiClient.get(`/api/web-articles/${id}`);
    },

    // Create new web article
    create: async (articleData) => {
        return apiClient.post('/api/web-articles', articleData);
    },

    // Update web article
    update: async (id, articleData) => {
        return apiClient.put(`/api/web-articles/${id}`, articleData);
    },

    // Delete web article
    delete: async (id) => {
        return apiClient.delete(`/api/web-articles/${id}`);
    },

    // Search web articles
    search: async (query, params = {}) => {
        return apiClient.get('/api/web-articles/search', { q: query, ...params });
    }
};

// ==========================================================================
// Memos API
// ==========================================================================

export const memosApi = {
    // Get memos for a paper
    getByPaperId: async (paperId) => {
        return apiClient.get(`/api/academic-papers/${paperId}/memos`);
    },

    // Create new memo
    create: async (paperId, memoData) => {
        return apiClient.post(`/api/academic-papers/${paperId}/memos`, memoData);
    },

    // Update memo
    update: async (paperId, memoId, memoData) => {
        return apiClient.put(`/api/academic-papers/${paperId}/memos/${memoId}`, memoData);
    },

    // Delete memo
    delete: async (paperId, memoId) => {
        return apiClient.delete(`/api/academic-papers/${paperId}/memos/${memoId}`);
    }
};

// ==========================================================================
// LLM/AI API
// ==========================================================================

export const llmApi = {
    // Generate memo using LLM
    generateMemo: async (paperData, userPrompt) => {
        return apiClient.post('/api/llm/generate-memo', {
            paper: paperData,
            prompt: userPrompt
        });
    },

    // Ask question about paper
    askQuestion: async (paperData, question) => {
        return apiClient.post('/api/llm/ask-question', {
            paper: paperData,
            question: question
        });
    }
};

// ==========================================================================
// Health Check API
// ==========================================================================

export const healthApi = {
    // Check API health
    check: async () => {
        return apiClient.get('/api/health');
    }
};

// ==========================================================================
// Error Handling Utilities
// ==========================================================================

export class ApiError extends Error {
    constructor(message, status, data = null) {
        super(message);
        this.name = 'ApiError';
        this.status = status;
        this.data = data;
    }
}

// Utility function to handle API errors
export const handleApiError = (error) => {
    if (error instanceof ApiError) {
        return error;
    }

    if (error.message === 'Request timeout') {
        return new ApiError('Request timed out. Please try again.', 408);
    }

    if (error.message.includes('HTTP error!')) {
        const status = parseInt(error.message.match(/status: (\d+)/)?.[1] || '500');
        return new ApiError('API request failed', status);
    }

    if (error.message.includes('Failed to fetch')) {
        return new ApiError('Network error. Please check your connection.', 0);
    }

    return new ApiError('An unexpected error occurred', 500, error);
};

// ==========================================================================
// Export default API client for custom usage
// ==========================================================================

export default apiClient;