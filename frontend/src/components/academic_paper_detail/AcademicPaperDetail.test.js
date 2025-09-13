import '@testing-library/jest-dom';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import AcademicPaperDetail from './AcademicPaperDetail';

// React Router useParamsのモック
jest.mock('react-router-dom', () => ({
    ...jest.requireActual('react-router-dom'),
    useParams: () => ({ id: '1' })
}));

// fetchのモック
global.fetch = jest.fn();

const mockPaper = {
    id: 1,
    title: 'Test Paper Title',
    authors: [
        {
            name: 'Test Author 1',
            h_index: 25,
            link: 'https://example.com/author1'
        },
        {
            name: 'Test Author 2',
            h_index: 30,
            link: 'https://example.com/author2'
        }
    ],
    abstract: 'This is a test abstract for the paper.',
    url: 'https://example.com/paper',
    published_date: '2023-01-15',
    journal: 'Test Journal',
    keywords: ['Test', 'Paper', 'Research'],
    content: {
        background: 'Test background content',
        research_question: 'Test research question',
        related_work: 'Test related work content',
        dataset: 'Test dataset description',
        experiment: 'Test experiment description',
        future_works: 'Test future works content'
    }
};

const renderWithRouter = (component) => {
    return render(
        <BrowserRouter>
            {component}
        </BrowserRouter>
    );
};

describe('AcademicPaperDetail', () => {
    beforeEach(() => {
        fetch.mockClear();
        window.open = jest.fn();
    });

    afterEach(() => {
        jest.resetAllMocks();
    });

    test('初期レンダリング時にローディング状態を表示する', () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPaper,
        });

        renderWithRouter(<AcademicPaperDetail />);
        
        expect(screen.getByText('論文の詳細情報を読み込み中...')).toBeInTheDocument();
    });

    test('論文の詳細情報を正常に取得して表示する', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPaper,
        });

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText('Test Paper Title')).toBeInTheDocument();
        });

        expect(screen.getByText('Test Author 1')).toBeInTheDocument();
        expect(screen.getByText('Test Author 2')).toBeInTheDocument();
        expect(screen.getByText('H-Index: 25')).toBeInTheDocument();
        expect(screen.getByText('H-Index: 30')).toBeInTheDocument();
        expect(screen.getByText('This is a test abstract for the paper.')).toBeInTheDocument();
        expect(screen.getByText('Test Journal')).toBeInTheDocument();
    });

    test('キーワードが正しく表示される', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPaper,
        });

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText('Test')).toBeInTheDocument();
        });

        expect(screen.getByText('Paper')).toBeInTheDocument();
        expect(screen.getByText('Research')).toBeInTheDocument();
    });

    test('論文の内容セクションが正しく表示される', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPaper,
        });

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText('論文の内容')).toBeInTheDocument();
        });

        expect(screen.getByText('背景とリサーチクエスチョン')).toBeInTheDocument();
        expect(screen.getByText('先行研究について')).toBeInTheDocument();
        expect(screen.getByText('データセットについて')).toBeInTheDocument();
        expect(screen.getByText('実験の概要と結果について')).toBeInTheDocument();
        expect(screen.getByText('Future Works')).toBeInTheDocument();
    });

    test('日付フォーマットが正しく表示される', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPaper,
        });

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText('2023年1月15日')).toBeInTheDocument();
        });
    });

    test('著者リンクが正しく設定される', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPaper,
        });

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            const authorLinks = screen.getAllByRole('link');
            const author1Link = authorLinks.find(link => 
                link.getAttribute('href') === 'https://example.com/author1'
            );
            expect(author1Link).toBeInTheDocument();
        });
    });

    test('論文リンクが正しく設定される', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPaper,
        });

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            const paperLink = screen.getByText('論文を読む');
            expect(paperLink.closest('a')).toHaveAttribute('href', 'https://example.com/paper');
        });
    });

    test('APIエラー時にエラーメッセージを表示する', async () => {
        fetch.mockRejectedValueOnce(new Error('API Error'));

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText(/エラー: API Error/)).toBeInTheDocument();
        });

        expect(screen.getByText('再試行')).toBeInTheDocument();
    });

    test('レスポンスエラー時にエラーメッセージを表示する', async () => {
        fetch.mockResolvedValueOnce({
            ok: false,
            status: 404,
        });

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText(/エラー: 論文の詳細情報の取得に失敗しました/)).toBeInTheDocument();
        });
    });

    test('再試行ボタンをクリックしてデータを再取得する', async () => {
        fetch.mockRejectedValueOnce(new Error('API Error'));

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText('再試行')).toBeInTheDocument();
        });

        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPaper,
        });

        fireEvent.click(screen.getByText('再試行'));

        await waitFor(() => {
            expect(screen.getByText('Test Paper Title')).toBeInTheDocument();
        });
    });

    test('開発環境でダミーデータを使用する', async () => {
        const originalEnv = process.env.NODE_ENV;
        process.env.NODE_ENV = 'development';

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText('Attention Is All You Need')).toBeInTheDocument();
        });

        process.env.NODE_ENV = originalEnv;
    });

    test('論文が見つからない場合の表示', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => null,
        });

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText('論文が見つかりませんでした')).toBeInTheDocument();
        });
    });

    test('論文の内容が正しく表示される', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPaper,
        });

        renderWithRouter(<AcademicPaperDetail />);

        await waitFor(() => {
            expect(screen.getByText('Test background content')).toBeInTheDocument();
        });

        expect(screen.getByText('Test research question')).toBeInTheDocument();
        expect(screen.getByText('Test related work content')).toBeInTheDocument();
        expect(screen.getByText('Test dataset description')).toBeInTheDocument();
        expect(screen.getByText('Test experiment description')).toBeInTheDocument();
        expect(screen.getByText('Test future works content')).toBeInTheDocument();
    });
});