import { Route, BrowserRouter as Router, Routes } from 'react-router-dom';
import './App.css';
import AcademicPaperDetail from './components/academic_paper_detail/AcademicPaperDetail';
import AcademicPaperTable from './components/academic_paper_table/AcademicPaperTable';
import Header from './components/header/Header';
import WebArticleTable from './components/web_article_table/WebArticleTable';
import logo from './logo.svg';

function App() {
    return (
        <Router>
            <div className="App">
                <Header />
                <Routes>
                    <Route path="/" element={
                        <header className="App-header">
                            <img src={logo} className="App-logo" alt="logo" />
                            <p>
                                Edit <code>src/App.js</code> and save to reload.
                            </p>
                            <a
                                className="App-link"
                                href="https://reactjs.org"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                Learn React
                            </a>
                        </header>
                    } />
                    <Route path="/papers" element={<AcademicPaperTable />} />
                    <Route path="/papers/:paper_id" element={<AcademicPaperDetail />} />
                    <Route path="/articles" element={<WebArticleTable />} />
                </Routes>
            </div>
        </Router>
    );
}

export default App;
