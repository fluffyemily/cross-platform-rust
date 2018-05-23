/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit

class ToDoListLabelsViewController: UITableViewController {

    var labels: [Label] = []

    lazy var dbStore: ToodleLib = {
        return ToodleLib.sharedInstance
    }()

    lazy var newLabelAlertController: UIAlertController = {
        let alert = UIAlertController(title: "New Label", message: "Enter label name", preferredStyle: .alert)
        let createAction = UIAlertAction(title: "Create", style: .default, handler: { (action) in
            guard let labelName = alert.textFields?[0].text else { return }
            self.createLabel(labelName: labelName)
        })
        alert.addAction(createAction)
        alert.addAction(UIAlertAction(title: "Cancel", style: .cancel, handler: { (action) in
        }))
        alert.addTextField { (textField) in
            textField.autocapitalizationType = .words
            textField.delegate = self as? UITextFieldDelegate
        }
        return alert
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        self.title = "Toodle"
        labels = self.dbStore.list.allLabels()
        self.tableView.register(UITableViewCell.self, forCellReuseIdentifier: "ListCell")
         self.navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: UIBarButtonSystemItem.add, target: self, action: #selector(newLabel))
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

    override func numberOfSections(in tableView: UITableView) -> Int {
        return 1
    }

    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return labels.count
    }

    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "ListCell", for: indexPath)
        let label = self.labels[indexPath.row]
        cell.contentView.backgroundColor = label.color
        cell.textLabel?.text = label.name
        return cell
    }

//    override func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
//        let itemsVC = ToDoListItemsTableViewController(label: self.labels[indexPath.row])
//        self.navigationController?.pushViewController(itemsVC, animated: true)
//    }

    @objc fileprivate func newLabel() {
        self.present(newLabelAlertController, animated: true)
    }

    fileprivate func createLabel(labelName: String) {
        let label = self.dbStore.list.createLabel(withName: labelName, color: UIColor.gray)
        self.labels.append(label)
        self.tableView.reloadData()
    }
}

extension ToDoListLabelsViewController: UITextViewDelegate {

    func textViewDidChange(_ textView: UITextView) {
        guard let text = textView.text,
            !text.isEmpty else {
                self.newLabelAlertController.actions[0].isEnabled = false
                return
        }
        self.newLabelAlertController.actions[0].isEnabled = true
    }
}
