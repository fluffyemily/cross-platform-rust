/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit

class ItemViewController: UIViewController {

    var delegate: ToDoListItemsViewControllerDelegate?

    lazy var itemDescriptionLabel: UILabel = {
        let label = UILabel()
        label.text = "Description:"
        label.textAlignment = .right
        return label
    }()

    var descriptionField: UITextField = {
        let textField = UITextField()
        textField.autocapitalizationType = .sentences
        textField.borderStyle = .bezel
        return textField
    }()


    lazy var dueDateLabel: UILabel = {
        let label = UILabel()
        label.text = "Due date:"
        label.textAlignment = .right
        return label
    }()

    var dueDateButton: UIButton = {
        let button = UIButton(type: .custom)
        button.setTitleColor(.blue, for: .normal)
        button.addTarget(self, action: #selector(togglePicker), for: .touchUpInside)
        return button
    }()

    var dueDatePicker: UIDatePicker = {
        let datePicker = UIDatePicker()
        datePicker.datePickerMode = .dateAndTime
        datePicker.minimumDate = Date()
        datePicker.addTarget(self, action: #selector(datePickerValueDidChange), for: UIControlEvents.valueChanged)
        return datePicker
    }()

    lazy var statusLabel: UILabel = {
        let label = UILabel()
        label.text = "Item status:"
        label.textAlignment = .right
        return label
    }()

    lazy var statusValueLabel: UILabel = {
        let label = UILabel()
        label.textAlignment = .right
        return label
    }()

    var completeButton: UIButton = {
        let button = UIButton(type: .custom)
        button.setTitle("Mark Complete", for: .normal)
        button.setTitleColor(.blue, for: .normal)
        button.addTarget(self, action: #selector(complete), for: .touchUpInside)
        return button
    }()

    var dueDatePickerHeightConstraint: NSLayoutConstraint?
    var dueDatePickerTopAnchorConstraint: NSLayoutConstraint?

    var item:Item?
    var category: Category

    init(category: Category) {
        self.category = category
        super.init(nibName: nil, bundle: nil)
        self.markComplete(isComplete: false)
        self.dueDateButton.setTitle("Set", for: .normal)
    }

    init(item: Item, category: Category) {
        self.category = category
        self.item = item
        super.init(nibName: nil, bundle: nil)

        self.descriptionField.text = item.description
        if let dueDate = item.dueDate {
            self.dueDateButton.setTitle(self.dateAsString(date: dueDate), for: .normal)
            self.dueDatePicker.date = dueDate
        }
        self.markComplete(isComplete: !item.isComplete)
    }

    required init?(coder aDecoder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    func dateAsString(date: Date) -> String {
        let dateFormatter = DateFormatter()
        dateFormatter.dateStyle = .long
        dateFormatter.timeStyle = .short
        return dateFormatter.string(from:date)
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        self.view.backgroundColor = .white

        if self.item == nil {
            self.title = "New Item"
            self.navigationItem.leftBarButtonItem = UIBarButtonItem(barButtonSystemItem: UIBarButtonSystemItem.cancel, target: self, action: #selector(close))
        } else {
            self.title = "Edit Item"
        }

        self.navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: UIBarButtonSystemItem.done, target: self, action: #selector(done))

        var constraints = [NSLayoutConstraint]()

        view.addSubview(itemDescriptionLabel)
        constraints += [itemDescriptionLabel.topAnchor.constraint(equalTo: view.topAnchor, constant: 100),
                        itemDescriptionLabel.leftAnchor.constraint(equalTo: view.leftAnchor, constant: 20),
                        itemDescriptionLabel.widthAnchor.constraint(equalToConstant: 98)]

        view.addSubview(descriptionField)
        constraints += [descriptionField.topAnchor.constraint(equalTo: itemDescriptionLabel.topAnchor),
                        descriptionField.leftAnchor.constraint(equalTo: itemDescriptionLabel.rightAnchor, constant: 20),
                        descriptionField.rightAnchor.constraint(equalTo: view.rightAnchor, constant: -20)]

        view.addSubview(dueDateLabel)
        constraints += [dueDateLabel.topAnchor.constraint(equalTo: itemDescriptionLabel.bottomAnchor, constant: 20),
                        dueDateLabel.leftAnchor.constraint(equalTo: view.leftAnchor, constant: 20),
                        dueDateLabel.widthAnchor.constraint(equalToConstant: 98)]

        view.addSubview(dueDateButton)
        constraints += [dueDateButton.centerYAnchor.constraint(equalTo: dueDateLabel.centerYAnchor),
                        dueDateButton.leftAnchor.constraint(equalTo: dueDateLabel.rightAnchor, constant: 10),
                        dueDateButton.widthAnchor.constraint(equalToConstant: 250)]

        view.addSubview(dueDatePicker)
        dueDatePickerHeightConstraint = dueDatePicker.heightAnchor.constraint(equalToConstant: 0)
        dueDatePickerTopAnchorConstraint = dueDatePicker.topAnchor.constraint(equalTo: dueDateButton.bottomAnchor, constant: 0)
        constraints += [dueDatePickerTopAnchorConstraint!,
                        dueDatePicker.leftAnchor.constraint(equalTo: dueDateLabel.rightAnchor),
                        dueDatePicker.rightAnchor.constraint(equalTo: view.rightAnchor, constant: -40),
                        dueDatePickerHeightConstraint!]

        view.addSubview(statusLabel)
        constraints += [statusLabel.topAnchor.constraint(equalTo: dueDatePicker.bottomAnchor, constant: 20),
                        statusLabel.leftAnchor.constraint(equalTo: view.leftAnchor, constant: 20),
                        statusLabel.widthAnchor.constraint(equalToConstant: 98)]

        view.addSubview(statusValueLabel)
        constraints += [statusValueLabel.topAnchor.constraint(equalTo: statusLabel.topAnchor),
                        statusValueLabel.leftAnchor.constraint(equalTo: statusLabel.rightAnchor, constant: 20)]

        view.addSubview(completeButton)
        constraints += [completeButton.topAnchor.constraint(equalTo: statusValueLabel.bottomAnchor, constant: 20),
                        completeButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        completeButton.widthAnchor.constraint(equalToConstant: 200)]

        NSLayoutConstraint.activate(constraints, translatesAutoresizingMaskIntoConstraints: false)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

    @objc func complete() {
        self.markComplete(isComplete: (self.statusValueLabel.text ?? "") != "Complete")
    }

    func markComplete(isComplete: Bool) {
        if isComplete {
            self.completeButton.isEnabled = false
            self.statusValueLabel.text = "Complete"
            self.statusValueLabel.textColor = .green
        } else {
            self.completeButton.isEnabled = true
            self.statusValueLabel.text = "Not yet complete"
        }
    }

    @objc func datePickerValueDidChange() {
        self.dueDateButton.setTitle(self.dateAsString(date: self.dueDatePicker.date), for: .normal)
    }

    @objc func togglePicker() {
        if (dueDatePickerHeightConstraint?.constant ?? 0) == 0 {
            self.dueDatePickerHeightConstraint?.constant = 150
            self.dueDatePickerTopAnchorConstraint?.constant = 20
        } else {
            self.dueDatePickerHeightConstraint?.constant = 0
            self.dueDatePickerTopAnchorConstraint?.constant = 0
        }
        self.view.updateConstraintsIfNeeded()
    }

    @objc func done() {
        self.save()
        if let _ = self.item {
            self.navigationController?.popViewController(animated: true)
        } else {
            self.close()
        }
    }

    @objc func close() {
        self.dismiss(animated: true, completion: nil)
    }

    func save() {
        guard let description = self.descriptionField.text else {
            return self.descriptionField.layer.borderColor = UIColor.red.cgColor
        }

        let currentItem = self.item ?? Item()

        currentItem.description = description
        if self.dueDateButton.titleLabel?.text != "Set" {
            currentItem.dueDate = self.dueDatePicker.date
        }
        currentItem.isComplete = (self.statusValueLabel.text ?? "") != "Complete"

        if let _ = currentItem.id {
            try? ToodleLib.sharedInstance.list.update(item: currentItem)
        } else {
            ToodleLib.sharedInstance.list.add(item: currentItem, toCategory: category)
        }
        self.delegate?.itemSaveSuccess(item: currentItem)
    }

}
