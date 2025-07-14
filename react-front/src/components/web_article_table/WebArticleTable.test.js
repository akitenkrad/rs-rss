import '@testing-library/jest-dom';
import { render, screen, waitFor } from '@testing-library/react';
import WebArticleTable from './WebArticleTable';

// Mock data
const mockArticles = [
    {
        id: 1,
        date: '2025-07-13',
        site_name: 'TechCrunch',
        title: 'Latest AI Developments in 2025',
        summary: 'A comprehensive overview of the latest artificial intelligence developments.',
        url: 'https://techcrunch.com/ai-developments-2025'
    },
    {
        id: 2,
        date: '2025-07-12',
        site_name: 'The Verge',
        title: 'Web Development Trends',
        summary: 'Exploring the newest trends in web development.',
        url: 'https://theverge.com/web-development-trends'
    }
];

describe('WebArticleTable', () => {
    test('renders page title', () => {
        render(<WebArticleTable />);
        expect(screen.getByText('Web Articles')).toBeInTheDocument();
    });

    test('shows loading state initially', () => {
        render(<WebArticleTable />);
        expect(screen.getByText('Loading articles...')).toBeInTheDocument();
    });

    test('renders table headers', async () => {
        render(<WebArticleTable />);
        
        await waitFor(() => {
            expect(screen.getByText('日付')).toBeInTheDocument();
            expect(screen.getByText('サイト名')).toBeInTheDocument();
            expect(screen.getByText('記事タイトル')).toBeInTheDocument();
            expect(screen.getByText('要約')).toBeInTheDocument();
            expect(screen.getByText('URL')).toBeInTheDocument();
        });
    });

    test('renders article data after loading', async () => {
        render(<WebArticleTable />);
        
        await waitFor(() => {
            expect(screen.getByText('Latest AI Developments in 2025')).toBeInTheDocument();
            expect(screen.getByText('TechCrunch')).toBeInTheDocument();
            expect(screen.getByText('Web Development Trends')).toBeInTheDocument();
            expect(screen.getByText('The Verge')).toBeInTheDocument();
        });
    });

    test('renders article links with correct attributes', async () => {
        render(<WebArticleTable />);
        
        await waitFor(() => {
            const links = screen.getAllByText('記事を読む');
            expect(links).toHaveLength(3); // Mock data has 3 articles
            
            links.forEach(link => {
                expect(link).toHaveAttribute('target', '_blank');
                expect(link).toHaveAttribute('rel', 'noopener noreferrer');
            });
        });
    });

    test('formats dates correctly', async () => {
        render(<WebArticleTable />);
        
        await waitFor(() => {
            expect(screen.getByText('2025/07/13')).toBeInTheDocument();
            expect(screen.getByText('2025/07/12')).toBeInTheDocument();
        });
    });

    test('applies correct CSS classes', () => {
        render(<WebArticleTable />);
        
        const container = document.querySelector('.web-article-table-container');
        expect(container).toBeInTheDocument();
        
        const title = screen.getByText('Web Articles');
        expect(title).toHaveClass('page-title');
    });

    test('renders table with correct structure', async () => {
        render(<WebArticleTable />);
        
        await waitFor(() => {
            const table = screen.getByRole('table');
            expect(table).toBeInTheDocument();
            expect(table).toHaveAttribute('aria-label', 'web articles table');
        });
    });
});