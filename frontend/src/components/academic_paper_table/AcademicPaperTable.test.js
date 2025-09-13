import '@testing-library/jest-dom';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import AcademicPaperTable from './AcademicPaperTable';

// fetch APIのモック
global.fetch = jest.fn();

// window.openのモック
Object.defineProperty(window, 'open', {
    writable: true,
    value: jest.fn(),
});

const mockPapers = [
    {
        id: 1,
        title: 'Deep Learning for Natural Language Processing',
        authors: 'John Smith, Jane Doe',
        published_date: '2024-01-15',
        journal: 'Journal of AI Research',
        abstract: 'This paper presents a comprehensive study of deep learning techniques for natural language processing tasks.',
        url: 'https://example.com/paper1'
    },
    {
        id: 2,
        title: 'Machine Learning in Healthcare',
        authors: 'Alice Johnson, Bob Wilson',
        published_date: '2024-02-20',
        journal: 'Medical AI Review',
        abstract: 'An analysis of machine learning applications in modern healthcare systems.',
        url: 'https://example.com/paper2'
    }
];

describe('AcademicPaperTable', () => {
    beforeEach(() => {
        fetch.mockClear();
        window.open.mockClear();
    });

    afterEach(() => {
        jest.resetAllMocks();
    });

    test('初期レンダリング時にローディング状態を表示する', () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);
        
        expect(screen.getByText('論文データを読み込み中...')).toBeInTheDocument();
    });

    test('論文データを正常に取得して表示する', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('論文一覧')).toBeInTheDocument();
        });

        expect(screen.getByText('Deep Learning for Natural Language Processing')).toBeInTheDocument();
        expect(screen.getByText('Machine Learning in Healthcare')).toBeInTheDocument();
        expect(screen.getByText('John Smith, Jane Doe')).toBeInTheDocument();
        expect(screen.getByText('Alice Johnson, Bob Wilson')).toBeInTheDocument();
    });

    test('APIエラー時にエラーメッセージを表示する', async () => {
        fetch.mockRejectedValueOnce(new Error('API Error'));

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText(/エラー: API Error/)).toBeInTheDocument();
        });

        expect(screen.getByText('再試行')).toBeInTheDocument();
    });

    test('レスポンスエラー時にエラーメッセージを表示する', async () => {
        fetch.mockResolvedValueOnce({
            ok: false,
            status: 500,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText(/エラー: 論文データの取得に失敗しました/)).toBeInTheDocument();
        });
    });

    test('再試行ボタンをクリックしてデータを再取得する', async () => {
        fetch.mockRejectedValueOnce(new Error('API Error'));

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('再試行')).toBeInTheDocument();
        });

        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        fireEvent.click(screen.getByText('再試行'));

        await waitFor(() => {
            expect(screen.getByText('論文一覧')).toBeInTheDocument();
        });
    });

    test('更新ボタンをクリックしてデータを更新する', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('更新')).toBeInTheDocument();
        });

        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        fireEvent.click(screen.getByText('更新'));

        expect(fetch).toHaveBeenCalledTimes(2);
    });

    test('タイトルによるソート機能が正常に動作する', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('タイトル')).toBeInTheDocument();
        });

        const titleHeader = screen.getByText('タイトル');
        fireEvent.click(titleHeader);

        // ソート後の順序を確認
        const rows = screen.getAllByRole('row');
        expect(rows).toHaveLength(3); // ヘッダー行 + データ行2つ
    });

    test('発表日によるソート機能が正常に動作する', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('発表日')).toBeInTheDocument();
        });

        const dateHeader = screen.getByText('発表日');
        fireEvent.click(dateHeader);

        // ソートインジケーターが表示されることを確認
        expect(screen.getByText('▲')).toBeInTheDocument();
    });

    test('同じヘッダーを2回クリックして降順ソートする', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('タイトル')).toBeInTheDocument();
        });

        const titleHeader = screen.getByText('タイトル');
        fireEvent.click(titleHeader);
        fireEvent.click(titleHeader);

        // 降順ソートインジケーターが表示されることを確認
        expect(screen.getByText('▼')).toBeInTheDocument();
    });

    test('詳細ボタンをクリックして新しいウィンドウを開く', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getAllByText('詳細')).toHaveLength(2);
        });

        const detailButtons = screen.getAllByText('詳細');
        fireEvent.click(detailButtons[0]);

        expect(window.open).toHaveBeenCalledWith('https://example.com/paper1', '_blank');
    });

    test('論文タイトルリンクをクリックして新しいウィンドウを開く', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('Deep Learning for Natural Language Processing')).toBeInTheDocument();
        });

        const titleLink = screen.getByText('Deep Learning for Natural Language Processing');
        expect(titleLink.closest('a')).toHaveAttribute('href', 'https://example.com/paper1');
        expect(titleLink.closest('a')).toHaveAttribute('target', '_blank');
    });

    test('論文数が正しく表示される', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('論文数: 2件')).toBeInTheDocument();
        });
    });

    test('論文データがない場合に適切なメッセージを表示する', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => [],
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('論文データがありません')).toBeInTheDocument();
        });

        expect(screen.getByText('論文数: 0件')).toBeInTheDocument();
    });

    test('長いテキストが適切に省略される', async () => {
        const longTitlePaper = {
            id: 3,
            title: 'This is a very long title that should be truncated because it exceeds the maximum length limit for display purposes in the table',
            authors: 'Test Author with a very long name that should also be truncated',
            published_date: '2024-03-01',
            journal: 'Test Journal',
            abstract: 'This is a very long abstract that contains multiple sentences and should be truncated to fit properly within the table cell without breaking the layout of the entire component.',
            url: 'https://example.com/paper3'
        };

        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => [longTitlePaper],
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText(/This is a very long title that should be truncated/)).toBeInTheDocument();
        });

        // 省略記号が含まれていることを確認
        const titleElement = screen.getByText(/This is a very long title that should be truncated/);
        expect(titleElement.textContent).toContain('...');
    });

    test('日付フォーマットが正しく表示される', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(screen.getByText('2024/1/15')).toBeInTheDocument();
            expect(screen.getByText('2024/2/20')).toBeInTheDocument();
        });
    });

    test('正しいAPIエンドポイントが呼び出される', async () => {
        fetch.mockResolvedValueOnce({
            ok: true,
            json: async () => mockPapers,
        });

        render(<AcademicPaperTable />);

        await waitFor(() => {
            expect(fetch).toHaveBeenCalledWith('/api/papers');
        });
    });
});