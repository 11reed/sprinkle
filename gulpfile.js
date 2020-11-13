const gulp = require('gulp');
const sass = require('gulp-sass');
const cleanCSS = require('gulp-clean-css');
const sizereport = require('gulp-sizereport');

sass.compiler = require('node-sass');

const path = {
    sass: './sass/**/*.scss',
    css: './css'
}

function style() {
    return gulp.src(path.sass)
    .pipe(sass.sync().on('error', sass.logError))
    .pipe(cleanCSS({compatibility: 'ie8'}))
    .pipe(sizereport({gzip: true}))
    .pipe(gulp.dest(path.css));
}

function watchTask() {
    gulp.watch(path.sass, style);
}

const build  = gulp.series(gulp.parallel(style));

exports.style = style;
exports.watchTask = watchTask;
exports.default = build;